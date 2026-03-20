use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use chrono::Utc;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::game::actions::{Action, ActionResult};
use crate::game::events;
use crate::game::evolution;
use crate::game::pet;
use crate::game::time;
use crate::game::voice;
use crate::save;
use crate::save::schema::{AlbumEntry, SaveData};
#[cfg(debug_assertions)]
use crate::ui::debug_album;
use crate::ui::{album, ascii_art, main_screen, naming};

pub enum AppMode {
    Startup,
    Naming {
        input: String,
        farewell_name: Option<String>,
    },
    Main,
    ActionAnimation,
    ActionReaction,
    Album,
    #[cfg(debug_assertions)]
    DebugAlbum,
    Death,
    Evolution,
}

pub struct StartupInfo {
    pub api_success: bool,
    pub drift_warning: Option<String>,
    pub elapsed_ticks: u64,
    pub elapsed_display: String,
    pub hatched_species: Option<String>,
    pub evolved_species: Option<String>,
    pub rollback_detected: bool,
    pub event_messages: Vec<String>,
    pub death_message: Option<String>,
}

pub struct AppState {
    pub mode: AppMode,
    pub save_data: SaveData,
    pub animation_frame: usize,
    pub last_frame_time: Instant,
    pub speech_text: String,
    pub startup_info: Option<StartupInfo>,
    pub action_result: Option<ActionResult>,
    pub action_animation_start: Option<Instant>,
    pub death_message: Option<String>,
    pub death_pet_name: Option<String>,
    pub evolution_message: Option<String>,
    pub album_state: album::AlbumState,
    #[cfg(debug_assertions)]
    pub debug_album_state: debug_album::DebugAlbumState,
    pub rng: ThreadRng,
}

fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}

pub async fn run() -> Result<()> {
    // 1. Fetch current time
    let time_result = time::fetch_current_time().await;
    let api_success = matches!(time_result.source, time::TimeSource::TimeApi);

    // 2. Load save data
    let existing_save = save::load()?;

    // 3. Build initial state
    let (mode, save_data, startup_info, startup_death_pet_name) = match existing_save {
        Some(mut data) => {
            let elapsed = time::calculate_elapsed_ticks(data.last_check_time, time_result.now);
            let mut rng = rand::thread_rng();
            let mut hatched_species = None;
            let mut evolved_species = None;
            let mut event_messages = Vec::new();
            let mut death_message = None;
            let mut death_pet_name: Option<String> = None;

            if !elapsed.rollback_detected {
                if let Some(ref mut p) = data.pet {
                    pet::apply_decay(p, elapsed.ticks, &mut rng);

                    // Check hatching (egg → stage 1)
                    let just_hatched = if let Some(hatch) = pet::check_hatching(p, &mut rng) {
                        hatched_species = Some(hatch.new_species);
                        true
                    } else {
                        false
                    };

                    // Check evolution (stage 1→2, 2→3, 3→4)
                    // Skip if just hatched to prevent cascading (e.g. egg→Stage1→Stage2 in one pass)
                    if !just_hatched {
                        if let Some(evo) = evolution::check_evolution(p, &mut rng) {
                            evolved_species = Some(evo.new_species);
                        }
                    }

                    // Process random events (includes accident/death check)
                    let event_results = events::process_offline_events(p, elapsed.ticks, &mut rng);
                    for er in event_results {
                        if er.is_death {
                            death_message = Some(er.message.clone());
                            // Save pet name for Death screen display
                            death_pet_name = Some(p.display_name().to_string());
                        }
                        event_messages.push(er.message);
                    }
                }

                // If death was detected, call record_death to maintain save data consistency
                if death_message.is_some() {
                    let death_msg = death_message.as_deref().unwrap_or("");
                    record_death(&mut data, death_msg);
                }

                data.last_check_time = time_result.now;
                save::save(&data)?;
            }

            let info = StartupInfo {
                api_success,
                drift_warning: time_result.drift_warning,
                elapsed_ticks: elapsed.ticks,
                elapsed_display: time::format_elapsed_short(elapsed.ticks),
                hatched_species,
                evolved_species,
                rollback_detected: elapsed.rollback_detected,
                event_messages,
                death_message,
            };

            (AppMode::Startup, data, Some(info), death_pet_name)
        }
        None => {
            let data = SaveData::new(time_result.now);
            (
                AppMode::Naming {
                    input: String::new(),
                    farewell_name: None,
                },
                data,
                None,
                None,
            )
        }
    };

    let speech_text = pick_idle_speech(&save_data, &mut rand::thread_rng());

    let mut state = AppState {
        mode,
        save_data,
        animation_frame: 0,
        last_frame_time: Instant::now(),
        speech_text,
        startup_info,
        action_result: None,
        action_animation_start: None,
        death_message: None,
        death_pet_name: startup_death_pet_name,
        evolution_message: None,
        album_state: album::AlbumState::new(),
        #[cfg(debug_assertions)]
        debug_album_state: debug_album::DebugAlbumState::new(),
        rng: rand::thread_rng(),
    };

    // 4. Initialize terminal (with panic hook for safe cleanup)
    install_panic_hook();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // 5. Event loop
    let result = run_loop(&mut terminal, &mut state).await;

    // 6. Cleanup terminal (always runs, even on error)
    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);

    // 7. Final save (best-effort on error path)
    if let Err(e) = save::save(&state.save_data) {
        eprintln!("セーブに失敗しました: {:#}", e);
    }

    result
}

async fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
) -> Result<()> {
    loop {
        // Render
        terminal.draw(|f| render(f, state))?;

        // Poll events (250ms timeout for animation updates)
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match handle_input(key.code, state)? {
                    InputResult::Continue => {}
                    InputResult::Quit => return Ok(()),
                }
            }
        }

        // Auto-transition: ActionAnimation → ActionReaction after 2.5s
        if matches!(state.mode, AppMode::ActionAnimation) {
            if let Some(start) = state.action_animation_start {
                if start.elapsed() >= Duration::from_millis(2500) {
                    state.action_animation_start = None;
                    state.mode = AppMode::ActionReaction;
                }
            }
        }

        // Update animation frame (~2fps)
        if state.last_frame_time.elapsed() >= Duration::from_millis(500) {
            state.animation_frame = state.animation_frame.wrapping_add(1);
            state.last_frame_time = Instant::now();
        }
    }
}

fn render(f: &mut ratatui::Frame, state: &AppState) {
    match &state.mode {
        AppMode::Startup => {
            main_screen::render_startup(f, state);
        }
        AppMode::Naming {
            input,
            farewell_name,
        } => {
            naming::render_naming_with_farewell(
                f,
                input,
                farewell_name.is_none(),
                farewell_name.as_deref(),
            );
        }
        AppMode::Main => {
            main_screen::render_main(f, state);
        }
        AppMode::ActionAnimation => {
            main_screen::render_action_animation(f, state);
        }
        AppMode::ActionReaction => {
            main_screen::render_action_reaction(f, state);
        }
        AppMode::Album => {
            album::render_album(f, &state.save_data, &state.album_state);
        }
        #[cfg(debug_assertions)]
        AppMode::DebugAlbum => {
            debug_album::render_debug_album(f, &state.debug_album_state, state.animation_frame);
        }
        AppMode::Death => {
            render_death(f, state);
        }
        AppMode::Evolution => {
            render_evolution(f, state);
        }
    }
}

fn render_death(f: &mut ratatui::Frame, state: &AppState) {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};
    let msg = state.death_message.as_deref().unwrap_or("…");

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  {}", msg),
        Style::default().fg(Color::Yellow),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(""));

    // Show farewell ASCII art
    lines.push(Line::from("        ．．．"));
    lines.push(Line::from(""));
    lines.push(Line::from(""));

    // Get pet name from death_pet_name (saved at startup) or from live pet data
    let pet_name = state
        .death_pet_name
        .as_deref()
        .or_else(|| state.save_data.pet.as_ref().map(|p| p.display_name()));

    if let Some(name) = pet_name {
        lines.push(Line::from(Span::styled(
            format!("  さよなら、{}。", name),
            Style::default().add_modifier(Modifier::BOLD),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Press any key...",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = ratatui::widgets::Paragraph::new(lines);
    f.render_widget(paragraph, f.area());
}

fn render_evolution(f: &mut ratatui::Frame, state: &AppState) {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};

    let msg = state.evolution_message.as_deref().unwrap_or("");

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    if let Some(ref pet) = state.save_data.pet {
        let name = pet.display_name();
        lines.push(Line::from(Span::styled(
            format!("  {}が、なにかに気づいたような顔をした。", name),
            Style::default().add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from("  ……"));
        lines.push(Line::from(""));
        lines.push(Line::from("  （しばらく、動かなかった）"));
        lines.push(Line::from(""));

        lines.push(Line::from(Span::styled(
            format!("  {}の様子が変わった。", name),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        // Show new species art
        let mood = pet::mood_level(pet.kimochi);
        let art = ascii_art::get_art(&pet.species, mood, state.animation_frame);
        for line in art {
            lines.push(Line::from(format!("        {}", line)));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  → {} に進化した！", msg),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Press any key...",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = ratatui::widgets::Paragraph::new(lines);
    f.render_widget(paragraph, f.area());
}

enum InputResult {
    Continue,
    Quit,
}

fn handle_input(key: KeyCode, state: &mut AppState) -> Result<InputResult> {
    match &mut state.mode {
        AppMode::Startup => {
            let startup = state.startup_info.take();

            // If death happened during startup, show death screen
            if let Some(ref info) = startup {
                if let Some(ref death_msg) = info.death_message {
                    state.death_message = Some(death_msg.clone());
                    state.mode = AppMode::Death;
                    return Ok(InputResult::Continue);
                }
            }

            // If pet is dead (from startup events), go to naming
            if state.save_data.pet.is_none() {
                state.mode = AppMode::Naming {
                    input: String::new(),
                    farewell_name: None,
                };
                return Ok(InputResult::Continue);
            }

            state.mode = AppMode::Main;
            state.speech_text = pick_idle_speech(&state.save_data, &mut state.rng);
        }
        AppMode::Naming { input, .. } => match key {
            KeyCode::Enter => {
                let nickname = if input.is_empty() {
                    String::new()
                } else {
                    input.clone()
                };

                let now = Utc::now();
                let new_pet = pet::new_egg(nickname, now);
                state.save_data.pet = Some(new_pet);
                state.save_data.last_check_time = now;
                state.save_data.records.total_monsters += 1;
                save::save(&state.save_data)?;

                state.mode = AppMode::Main;
                state.speech_text = pick_idle_speech(&state.save_data, &mut state.rng);
            }
            KeyCode::Char(c) => {
                if let AppMode::Naming { input, .. } = &mut state.mode {
                    if input.chars().count() < 20 {
                        input.push(c);
                    }
                }
            }
            KeyCode::Backspace => {
                if let AppMode::Naming { input, .. } = &mut state.mode {
                    input.pop();
                }
            }
            _ => {}
        },
        AppMode::Main => match key {
            KeyCode::Char('t') | KeyCode::Char('T') => {
                do_action(Action::Talk, state)?;
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                do_action(Action::Play, state)?;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                do_action(Action::Train, state)?;
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                do_action(Action::Relax, state)?;
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                state.album_state = album::AlbumState::new();
                state.mode = AppMode::Album;
            }
            #[cfg(debug_assertions)]
            KeyCode::Char('d') | KeyCode::Char('D') => {
                state.debug_album_state = debug_album::DebugAlbumState::new();
                state.mode = AppMode::DebugAlbum;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                return Ok(InputResult::Quit);
            }
            _ => {}
        },
        AppMode::ActionAnimation => {
            // Any key → skip animation, go straight to result
            state.action_animation_start = None;
            state.mode = AppMode::ActionReaction;
        }
        AppMode::ActionReaction => {
            // Any key → back to Main
            state.action_result = None;
            state.mode = AppMode::Main;
            state.speech_text = pick_idle_speech(&state.save_data, &mut state.rng);
        }
        AppMode::Album => match key {
            KeyCode::Up => {
                state.album_state.scroll_up();
            }
            KeyCode::Down => {
                let total = album::total_species_count() + 1; // +1 for mystery line
                state.album_state.scroll_down(total, 20);
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                state.mode = AppMode::Main;
                state.speech_text = pick_idle_speech(&state.save_data, &mut state.rng);
            }
            _ => {}
        },
        #[cfg(debug_assertions)]
        AppMode::DebugAlbum => {
            use debug_album::DebugAlbumView;
            let ds = &mut state.debug_album_state;
            match (&ds.view, key) {
                // ── List view ──
                (DebugAlbumView::List, KeyCode::Up) => {
                    ds.cursor_up();
                }
                (DebugAlbumView::List, KeyCode::Down) => {
                    let visible = 20; // approximate visible lines
                    ds.cursor_down(visible);
                }
                (DebugAlbumView::List, KeyCode::Enter) => {
                    ds.view = DebugAlbumView::Idle;
                }
                (DebugAlbumView::List, KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc) => {
                    state.mode = AppMode::Main;
                    state.speech_text = pick_idle_speech(&state.save_data, &mut state.rng);
                }
                // ── Idle view ──
                (DebugAlbumView::Idle, KeyCode::Left) => {
                    ds.prev_species();
                }
                (DebugAlbumView::Idle, KeyCode::Right) => {
                    ds.next_species();
                }
                (DebugAlbumView::Idle, KeyCode::Char('a') | KeyCode::Char('A')) => {
                    ds.view = DebugAlbumView::Action { index: 0 };
                }
                (DebugAlbumView::Idle, KeyCode::Esc) => {
                    ds.view = DebugAlbumView::List;
                }
                // ── Action view ──
                (DebugAlbumView::Action { .. }, KeyCode::Right) => {
                    ds.next_action();
                }
                (DebugAlbumView::Action { .. }, KeyCode::Left) => {
                    ds.prev_action();
                }
                (DebugAlbumView::Action { .. }, KeyCode::Char('i') | KeyCode::Char('I')) => {
                    ds.view = DebugAlbumView::Idle;
                }
                (DebugAlbumView::Action { .. }, KeyCode::Esc) => {
                    ds.view = DebugAlbumView::List;
                }
                _ => {}
            }
        }
        AppMode::Death => {
            // Process death: record to album, clear pet, go to naming
            // Get farewell name from death_pet_name (startup) or from live pet data
            let farewell = state.death_pet_name.take().or_else(|| {
                state
                    .save_data
                    .pet
                    .as_ref()
                    .map(|p| p.display_name().to_string())
            });

            // Record death if pet is still alive (startup deaths already recorded)
            if state.save_data.pet.is_some() {
                let death_msg = state.death_message.take().unwrap_or_default();
                record_death(&mut state.save_data, &death_msg);
                save::save(&state.save_data)?;
            }

            state.death_message = None;
            state.mode = AppMode::Naming {
                input: String::new(),
                farewell_name: farewell,
            };
        }
        AppMode::Evolution => {
            // Any key → back to Main
            state.evolution_message = None;
            state.mode = AppMode::Main;
            state.speech_text = pick_idle_speech(&state.save_data, &mut state.rng);
        }
    }

    Ok(InputResult::Continue)
}

fn do_action(action: Action, state: &mut AppState) -> Result<()> {
    if let Some(ref mut pet_data) = state.save_data.pet {
        // Pull-based time check: calculate elapsed since last check
        let now = Utc::now();
        let elapsed = time::calculate_elapsed_ticks(state.save_data.last_check_time, now);
        if !elapsed.rollback_detected && elapsed.ticks > 0 {
            pet::apply_decay(pet_data, elapsed.ticks, &mut state.rng);
        }
        state.save_data.last_check_time = now;

        // Check for hatching before action
        let just_hatched = pet::check_hatching(pet_data, &mut state.rng).is_some();

        // Check evolution (skip if just hatched to prevent cascading)
        if !just_hatched {
            if let Some(evo) = evolution::check_evolution(pet_data, &mut state.rng) {
                state.evolution_message = Some(evo.new_species);
                state.mode = AppMode::Evolution;
                save::save(&state.save_data)?;
                return Ok(());
            }
        }

        // Egg stage: ignore actions (prevent type score accumulation before hatching)
        if pet_data.stage == 0 {
            state.action_result = Some(ActionResult {
                action,
                reaction_text: "（たまごは静かに揺れている…）".to_string(),
            });
            state.action_animation_start = Some(Instant::now());
            state.mode = AppMode::ActionAnimation;
            return Ok(());
        }

        // Perform the action with voice-type-aware reactions
        let mood = pet::mood_level(pet_data.kimochi);
        crate::game::actions::apply_action_effects(action, pet_data, &mut state.rng);

        let reaction_text = if let Some(vt) = evolution::get_voice_type(&pet_data.species) {
            voice::get_reaction(vt, action, mood, &mut state.rng)
        } else {
            // Stage1 fallback: use Phase1 generic reactions
            crate::game::actions::select_generic_reaction(action, mood, &mut state.rng)
        };

        state.action_result = Some(ActionResult {
            action,
            reaction_text,
        });
        state.action_animation_start = Some(Instant::now());
        state.mode = AppMode::ActionAnimation;

        // Save after every action
        save::save(&state.save_data)?;
    }

    Ok(())
}

/// Record death to album and clear pet data
fn record_death(save_data: &mut SaveData, death_message: &str) {
    if let Some(pet) = save_data.pet.take() {
        let days_lived = (pet.age_ticks as f64 / 1440.0).ceil() as u32;
        let weight_label = pet::weight_label(&pet.species, pet.weight).to_string();

        // Update records
        if pet.age_ticks > save_data.records.longest_survival_ticks {
            save_data.records.longest_survival_ticks = pet.age_ticks;
        }

        let display = pet.display_name().to_string();
        let entry = AlbumEntry {
            nickname: display,
            species: pet.species,
            days_lived,
            weight_kg: pet.weight,
            weight_label,
            cause_of_death: death_message.to_string(),
            evolution_line: pet.evolution_line,
            reached_stage4: pet.stage >= 4,
            date: Utc::now().format("%Y-%m-%d").to_string(),
        };

        save_data.album.push(entry);
        save_data.pet = None;
    }
}

fn pick_idle_speech(save_data: &SaveData, rng: &mut impl Rng) -> String {
    if let Some(ref pet) = save_data.pet {
        let mood = pet::mood_level(pet.kimochi);

        // Stage2+: use voice-type-specific idle speech
        if let Some(vt) = evolution::get_voice_type(&pet.species) {
            return voice::get_idle_speech(vt, mood, rng);
        }

        // Stage1: use generic idle speech from Phase 1
        let pool = ascii_art::get_idle_speech(&pet.species, mood);
        match pool.choose(rng) {
            Some(s) => s.to_string(),
            None => String::new(),
        }
    } else {
        String::new()
    }
}

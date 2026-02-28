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
use rand::Rng;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::game::actions::{Action, ActionResult};
use crate::game::pet;
use crate::game::time;
use crate::save;
use crate::save::schema::SaveData;
use crate::ui::{ascii_art, main_screen, naming};

pub enum AppMode {
    Startup,
    Naming { input: String, is_first_launch: bool },
    Main,
    ActionReaction,
}

pub struct StartupInfo {
    pub api_success: bool,
    pub drift_warning: Option<String>,
    pub elapsed_ticks: u64,
    pub elapsed_display: String,
    pub hatched_species: Option<String>,
    pub rollback_detected: bool,
}

pub struct AppState {
    pub mode: AppMode,
    pub save_data: SaveData,
    pub animation_frame: usize,
    pub last_frame_time: Instant,
    pub speech_text: String,
    pub startup_info: Option<StartupInfo>,
    pub action_result: Option<ActionResult>,
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
    let api_success = matches!(time_result.source, time::TimeSource::WorldTimeApi);

    // 2. Load save data
    let existing_save = save::load()?;

    // 3. Build initial state
    let (mode, save_data, startup_info) = match existing_save {
        Some(mut data) => {
            let elapsed = time::calculate_elapsed_ticks(data.last_check_time, time_result.now);
            let mut rng = rand::thread_rng();
            let mut hatched_species = None;

            if !elapsed.rollback_detected {
                if let Some(ref mut p) = data.pet {
                    pet::apply_decay(p, elapsed.ticks, &mut rng);
                    if let Some(hatch) = pet::check_hatching(p, &mut rng) {
                        hatched_species = Some(hatch.new_species);
                    }
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
                rollback_detected: elapsed.rollback_detected,
            };

            (AppMode::Startup, data, Some(info))
        }
        None => {
            let data = SaveData::new(time_result.now);
            (
                AppMode::Naming {
                    input: String::new(),
                    is_first_launch: true,
                },
                data,
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

    // 6. Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    // 7. Final save
    save::save(&state.save_data)?;

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
        AppMode::Naming { input, is_first_launch } => {
            naming::render_naming(f, input, *is_first_launch);
        }
        AppMode::Main => {
            main_screen::render_main(f, state);
        }
        AppMode::ActionReaction => {
            main_screen::render_action_reaction(f, state);
        }
    }
}

enum InputResult {
    Continue,
    Quit,
}

fn handle_input(key: KeyCode, state: &mut AppState) -> Result<InputResult> {
    match &mut state.mode {
        AppMode::Startup => {
            // Any key → transition to Main
            state.startup_info = None;
            state.mode = AppMode::Main;
            state.speech_text = pick_idle_speech(&state.save_data, &mut state.rng);
        }
        AppMode::Naming { input, .. } => {
            match key {
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
                    // Safety: reconstruct mutable ref
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
            }
        }
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
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                return Ok(InputResult::Quit);
            }
            _ => {}
        },
        AppMode::ActionReaction => {
            // Any key → back to Main
            state.action_result = None;
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
        if let Some(_hatch) = pet::check_hatching(pet_data, &mut state.rng) {
            // Hatching happened! Will be visible on next render
        }

        // Egg stage: ignore actions (prevent type score accumulation before hatching)
        if pet_data.stage == 0 {
            state.action_result = Some(ActionResult {
                action,
                reaction_text: "（たまごは静かに揺れている…）".to_string(),
            });
            state.mode = AppMode::ActionReaction;
            return Ok(());
        }

        // Perform the action
        let result = crate::game::actions::perform_action(action, pet_data, &mut state.rng);

        state.action_result = Some(result);
        state.mode = AppMode::ActionReaction;

        // Save after every action
        save::save(&state.save_data)?;
    }

    Ok(())
}

fn pick_idle_speech(save_data: &SaveData, rng: &mut impl Rng) -> String {
    if let Some(ref pet) = save_data.pet {
        let mood = pet::mood_level(pet.kimochi);
        let pool = ascii_art::get_idle_speech(&pet.species, mood);
        if pool.is_empty() {
            return String::new();
        }
        let idx = rng.gen_range(0..pool.len());
        pool[idx].to_string()
    } else {
        String::new()
    }
}

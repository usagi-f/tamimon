use chrono::Utc;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;
use crate::game::actions::{Action, TRAIN_REPS};
use crate::game::evolution::{get_body_type, BodyType};
use crate::game::pet::{mood_level, weight_label};
use crate::game::time::format_elapsed;
use crate::ui::ascii_art;

/// Returns the horizontal sway offset (in extra leading spaces) for a species
/// based on its body type and the current animation frame.
/// Floaty types (Nagare, Fuwafuwa) gently sway ±1 char; others stay still.
fn sway_offset(body_type: BodyType, frame: usize) -> usize {
    match body_type {
        BodyType::Nagare | BodyType::Fuwafuwa => [0, 1, 2, 1][frame % 4],
        _ => 0,
    }
}

/// Apply blink to art lines: replace eye chars when blinking.
/// Blink triggers for 1 frame every ~4 seconds (every 8 frames at 2fps).
fn apply_blink(lines: Vec<String>, blink_tick: u32) -> Vec<String> {
    // Active for 1 frame every 8 frames (once per ~4s at 2fps)
    if !blink_tick.is_multiple_of(8) {
        return lines;
    }
    lines
        .into_iter()
        .map(|l| l.replace('ω', "－").replace('ᵕ', "＿").replace("◉", "ー"))
        .collect()
}

pub fn render_main(f: &mut Frame, state: &AppState) {
    let pet = match &state.save_data.pet {
        Some(p) => p,
        None => return,
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(8),    // body (art + speech)
            Constraint::Length(4), // footer
        ])
        .split(f.area());

    // --- Header ---
    let now = Utc::now();
    let age_str = format_elapsed(pet.birth_timestamp, now);
    let w_label = weight_label(&pet.species, pet.weight);
    let weight_str = format!("{:.0}kg ({})", pet.weight, w_label);
    let nickname = pet.display_name();

    let header_line = Line::from(vec![Span::styled(
        format!("  {}（{}）  ", nickname, pet.species),
        Style::default().add_modifier(Modifier::BOLD),
    )]);
    let header_right = Line::from(vec![Span::raw(format!("{}  ⚖ {}  ", age_str, weight_str))]);

    let header_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));

    let header_area = chunks[0];
    f.render_widget(header_block, header_area);

    let header_inner = super::inner_area(header_area, 0, 1);
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(header_inner);

    f.render_widget(
        Paragraph::new(header_line).alignment(Alignment::Left),
        header_chunks[0],
    );
    f.render_widget(
        Paragraph::new(header_right).alignment(Alignment::Right),
        header_chunks[1],
    );

    // --- Body ---
    let mood = mood_level(pet.kimochi);
    let raw_art = ascii_art::get_art(&pet.species, mood, state.animation_frame);

    // Apply blink (eye substitution) and horizontal sway by body type
    let art_lines = apply_blink(raw_art, state.blink_tick);
    let sway = get_body_type(&pet.species)
        .map(|bt| sway_offset(bt, state.animation_frame))
        .unwrap_or(0);
    let pad = " ".repeat(sway);

    let speech = &state.speech_text;
    let speech_line = format!("「 {} 」", speech);

    let mut body_lines: Vec<Line> = Vec::new();
    body_lines.push(Line::from(""));

    for art_line in art_lines {
        body_lines.push(Line::from(format!("{}{}", pad, art_line)));
    }

    body_lines.push(Line::from(""));
    body_lines.push(Line::from(Span::styled(
        speech_line,
        Style::default().fg(Color::Yellow),
    )));

    let body = Paragraph::new(body_lines).alignment(Alignment::Center);
    f.render_widget(body, chunks[1]);

    // --- Footer ---
    let footer_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));

    f.render_widget(footer_block, chunks[2]);

    let footer_inner = super::inner_area(chunks[2], 0, 1);
    #[cfg(not(debug_assertions))]
    let footer_lines = vec![
        Line::from("  [T]話しかける  [P]あそぶ  [R]特訓  [E]まったり"),
        Line::from("  [A]図鑑                                [Q]終了"),
    ];
    #[cfg(debug_assertions)]
    let footer_lines = vec![
        Line::from("  [T]話しかける  [P]あそぶ  [R]特訓  [E]まったり"),
        Line::from("  [A]図鑑  [D]デバッグ図鑑                [Q]終了"),
    ];
    let footer = Paragraph::new(footer_lines);
    f.render_widget(footer, footer_inner);
}

pub fn render_startup(f: &mut Frame, state: &AppState) {
    let info = match &state.startup_info {
        Some(i) => i,
        None => return,
    };

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Tamimon",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Cyan),
    )));
    lines.push(Line::from(""));

    let time_status = if info.api_success {
        "  Connecting to time server...  ✓"
    } else {
        "  Connecting to time server...  ✗ (ローカル時刻を使用)"
    };
    lines.push(Line::from(time_status));

    if let Some(warning) = &info.drift_warning {
        lines.push(Line::from(Span::styled(
            format!("  ⚠ {}", warning),
            Style::default().fg(Color::Yellow),
        )));
    }

    lines.push(Line::from("  Loading save data...          ✓"));

    if info.elapsed_ticks > 0 {
        lines.push(Line::from(""));
        lines.push(Line::from(format!("  経過時間: {}", info.elapsed_display)));
        lines.push(Line::from(""));

        if let Some(pet) = &state.save_data.pet {
            lines.push(Line::from(format!(
                "  {}は自由気ままに過ごしていたよ！",
                pet.display_name()
            )));
        }
    }

    if let Some(ref hatched) = info.hatched_species {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  🥚 たまごが かえった！ → {}", hatched),
            Style::default().fg(Color::Green),
        )));
    }

    // Only show evolution if the pet didn't die in the same startup
    if info.death_message.is_none() {
        if let Some(ref evolved) = info.evolved_species {
            lines.push(Line::from(""));
            let evo_msg = if let Some(ref pet) = state.save_data.pet {
                format!("  ✨ {}が進化した！ → {}", pet.display_name(), evolved)
            } else {
                format!("  ✨ 進化した！ → {}", evolved)
            };
            lines.push(Line::from(Span::styled(
                evo_msg,
                Style::default().fg(Color::Magenta),
            )));
        }
    }

    // Show non-death event messages (death message will be shown on the Death screen)
    let non_death_msgs: Vec<&String> = info
        .event_messages
        .iter()
        .filter(|m| info.death_message.as_ref() != Some(m))
        .collect();
    if !non_death_msgs.is_empty() {
        lines.push(Line::from(""));
        for msg in &non_death_msgs {
            lines.push(Line::from(Span::styled(
                format!("  {}", msg),
                Style::default().fg(Color::Yellow),
            )));
        }
    }

    if info.rollback_detected {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  ⚠ 時刻の巻き戻しを検知しました。ゲームは進行しません。",
            Style::default().fg(Color::Red),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Press any key to continue...",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, f.area());
}

pub fn render_action_animation(f: &mut Frame, state: &AppState) {
    let action = match &state.action_result {
        Some(r) => r.action,
        None => return,
    };

    let pet = match &state.save_data.pet {
        Some(p) => p,
        None => return,
    };

    let nickname = pet.display_name();

    let art_lines = ascii_art::get_action_art(&pet.species, action, state.animation_frame);

    let elapsed_ms = state
        .action_animation_start
        .map(|s| s.elapsed().as_millis() as u64)
        .unwrap_or(0);

    // For Relax: grow ～ slowly over 5s (no progress dots, non-skippable)
    // For others: use frame-based effect + progress dots
    let (effect, progress) = if action == Action::Relax {
        let count = ((elapsed_ms * 22 / 5000) as usize + 1).min(22);
        ("～".repeat(count), "")
    } else {
        let e = ascii_art::get_action_effect(action, state.animation_frame).to_string();
        let p = if elapsed_ms < 800 {
            "."
        } else if elapsed_ms < 1600 {
            ".."
        } else {
            "..."
        };
        (e, p)
    };

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  [{}] {}{}", action.key(), action.label(), progress),
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(format!("  {}の様子…", nickname)));
    lines.push(Line::from(""));

    for art_line in art_lines {
        lines.push(Line::from(format!("        {}", art_line)));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  {}", effect),
        Style::default().fg(Color::Cyan),
    )));

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, f.area());
}

pub fn render_action_reaction(f: &mut Frame, state: &AppState) {
    let result = match &state.action_result {
        Some(r) => r,
        None => return,
    };
    let action = result.action;
    let reaction_lines = &result.reaction_lines;
    let current_line = result.current_line;

    let pet = match &state.save_data.pet {
        Some(p) => p,
        None => return,
    };

    let nickname = pet.display_name();
    let mood = mood_level(pet.kimochi);
    let art_lines = ascii_art::get_art(&pet.species, mood, state.animation_frame);

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  [{}] {} を選んだ", action.key(), action.label()),
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    let facing = match action {
        Action::Train => format!("  {}が構えた。", nickname),
        _ => format!("  {}がこちらを向いた。", nickname),
    };
    lines.push(Line::from(facing));
    lines.push(Line::from(""));

    for art_line in art_lines {
        lines.push(Line::from(format!("        {}", art_line)));
    }

    lines.push(Line::from(""));

    match action {
        Action::Talk => {
            if let Some(pl) = &result.player_line {
                lines.push(Line::from(Span::styled(
                    format!("  あなた: {}", pl),
                    Style::default().fg(Color::Cyan),
                )));
            }
            if let Some(text) = reaction_lines.first() {
                lines.push(Line::from(Span::styled(
                    format!("  {}: {}", nickname, text),
                    Style::default().fg(Color::Yellow),
                )));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Press any key...",
                Style::default().fg(Color::DarkGray),
            )));
        }
        Action::Play => {
            let elapsed = state
                .reaction_anim_start
                .map(|s| s.elapsed().as_millis())
                .unwrap_or(u128::MAX);
            let revealed = ((elapsed / 600 + 1) as usize).min(reaction_lines.len());
            for text in reaction_lines.iter().take(revealed) {
                lines.push(Line::from(Span::styled(
                    format!("  {}", text),
                    Style::default().fg(Color::Yellow),
                )));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(""));
            if revealed >= reaction_lines.len() {
                lines.push(Line::from(Span::styled(
                    "  Press any key...",
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }
        Action::Train => {
            if current_line < TRAIN_REPS {
                // Effort phase: show current rep text and counter
                if let Some(text) = reaction_lines.get(current_line) {
                    lines.push(Line::from(Span::styled(
                        format!("  {}", text),
                        Style::default().fg(Color::Yellow),
                    )));
                }
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("  × {} / {}", current_line + 1, TRAIN_REPS),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "  → [any key] もう一回！",
                    Style::default().fg(Color::DarkGray),
                )));
            } else {
                // Completion phase: show completion text
                if let Some(text) = reaction_lines.get(TRAIN_REPS) {
                    lines.push(Line::from(Span::styled(
                        format!("  {}", text),
                        Style::default().fg(Color::Yellow),
                    )));
                }
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("  ✓ {} / {} 完了", TRAIN_REPS, TRAIN_REPS),
                    Style::default().fg(Color::Green),
                )));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "  Press any key...",
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }
        Action::Relax => {
            if let Some(text) = reaction_lines.first() {
                lines.push(Line::from(Span::styled(
                    format!("  {}", text),
                    Style::default().fg(Color::Yellow),
                )));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Press any key...",
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, f.area());
}

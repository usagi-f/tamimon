use chrono::Utc;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::AppState;
use crate::game::pet::{mood_level, weight_label};
use crate::game::time::format_elapsed;
use crate::ui::ascii_art;

pub fn render_main(f: &mut Frame, state: &AppState) {
    let pet = match &state.save_data.pet {
        Some(p) => p,
        None => return,
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header
            Constraint::Min(8),    // body (art + speech)
            Constraint::Length(4), // footer
        ])
        .split(f.area());

    // --- Header ---
    let now = Utc::now();
    let age_str = format_elapsed(pet.birth_timestamp, now);
    let w_label = weight_label(&pet.species, pet.weight);
    let weight_str = format!("{:.0}kg ({})", pet.weight, w_label);
    let nickname = if pet.nickname.is_empty() {
        "なまえなし"
    } else {
        &pet.nickname
    };

    let header_line = Line::from(vec![
        Span::styled(
            format!("  {}  ", nickname),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]);
    let header_right = Line::from(vec![Span::raw(format!(
        "{}  ⚖ {}  ",
        age_str, weight_str
    ))]);

    let header_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));

    let header_area = chunks[0];
    f.render_widget(header_block, header_area);

    let header_inner = inner_area(header_area, 0, 1);
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
    let art_lines = ascii_art::get_art(&pet.species, mood, state.animation_frame);

    let speech = &state.speech_text;
    let speech_line = format!("「 {} 」", speech);

    let mut body_lines: Vec<Line> = Vec::new();
    body_lines.push(Line::from(""));

    for art_line in art_lines {
        body_lines.push(Line::from(art_line.to_string()));
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

    let footer_inner = inner_area(chunks[2], 0, 1);
    let footer_lines = vec![
        Line::from("  [T]話しかける  [P]あそぶ  [R]特訓  [E]まったり"),
        Line::from("  [A]図鑑                                [Q]終了"),
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
        lines.push(Line::from(format!(
            "  経過時間: {}",
            info.elapsed_display
        )));
        lines.push(Line::from(""));

        if let Some(pet) = &state.save_data.pet {
            let name = if pet.nickname.is_empty() {
                "なまえなし"
            } else {
                &pet.nickname
            };
            lines.push(Line::from(format!(
                "  {}は自由気ままに過ごしていたよ！",
                name
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
                let name = if pet.nickname.is_empty() { "なまえなし" } else { &pet.nickname };
                format!("  ✨ {}が進化した！ → {}", name, evolved)
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
    let non_death_msgs: Vec<&String> = info.event_messages.iter()
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

pub fn render_action_animation(
    f: &mut Frame,
    state: &AppState,
) {
    let action = match &state.action_result {
        Some(r) => r.action,
        None => return,
    };

    let pet = match &state.save_data.pet {
        Some(p) => p,
        None => return,
    };

    let nickname = if pet.nickname.is_empty() {
        "なまえなし"
    } else {
        &pet.nickname
    };

    let (art_lines, effect) = ascii_art::get_action_animation(action, state.animation_frame);

    // Progress dots based on elapsed time
    let elapsed_ms = state.action_animation_start
        .map(|s| s.elapsed().as_millis() as u64)
        .unwrap_or(0);
    let progress = if elapsed_ms < 800 {
        "."
    } else if elapsed_ms < 1600 {
        ".."
    } else {
        "..."
    };

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  [{}] {}{}", action_key(action), action.label(), progress),
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "  {}の様子…",
        nickname
    )));
    lines.push(Line::from(""));

    for art_line in art_lines {
        lines.push(Line::from(format!("        {}", art_line)));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        effect.to_string(),
        Style::default().fg(Color::Cyan),
    )));

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, f.area());
}

pub fn render_action_reaction(
    f: &mut Frame,
    state: &AppState,
) {
    let (action, reaction_text) = match &state.action_result {
        Some(r) => (r.action, &r.reaction_text),
        None => return,
    };

    let pet = match &state.save_data.pet {
        Some(p) => p,
        None => return,
    };

    let nickname = if pet.nickname.is_empty() {
        "なまえなし"
    } else {
        &pet.nickname
    };

    let art_lines = ascii_art::get_action_art(&pet.species, action);

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  [{}] {} を選んだ", action_key(action), action.label()),
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "  {}がこちらを向いた。",
        nickname
    )));
    lines.push(Line::from(""));

    for art_line in art_lines {
        lines.push(Line::from(format!("        {}", art_line)));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  {}", reaction_text),
        Style::default().fg(Color::Yellow),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Press any key...",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, f.area());
}

fn action_key(action: crate::game::actions::Action) -> &'static str {
    match action {
        crate::game::actions::Action::Talk => "T",
        crate::game::actions::Action::Play => "P",
        crate::game::actions::Action::Train => "R",
        crate::game::actions::Action::Relax => "E",
    }
}

fn inner_area(area: Rect, h_margin: u16, v_margin: u16) -> Rect {
    Rect {
        x: area.x + h_margin,
        y: area.y + v_margin,
        width: area.width.saturating_sub(h_margin * 2),
        height: area.height.saturating_sub(v_margin * 2),
    }
}

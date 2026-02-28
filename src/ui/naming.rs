use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::ascii_art;

pub fn render_naming(f: &mut Frame, input: &str, is_first_launch: bool) {
    let egg = ascii_art::egg_art();

    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    if !is_first_launch {
        lines.push(Line::from(""));
        lines.push(Line::from("  ───────────────────────────────"));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        "  あたらしい たまごが やってきた。",
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    for art_line in egg {
        lines.push(Line::from(format!("      {}", art_line)));
    }

    lines.push(Line::from(""));
    lines.push(Line::from("  このこの なまえは？"));
    lines.push(Line::from(""));

    let cursor = if input.is_empty() {
        "  > █".to_string()
    } else {
        format!("  > {}█", input)
    };
    lines.push(Line::from(Span::styled(
        cursor,
        Style::default().fg(Color::Cyan),
    )));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  （Enterで決定・空欄でもOK）",
        Style::default().fg(Color::DarkGray),
    )));

    let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
    f.render_widget(paragraph, f.area());
}

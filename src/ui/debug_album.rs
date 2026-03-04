use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use crate::save::schema::SaveData;

pub fn render_debug_album(f: &mut Frame, _save_data: &SaveData) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let block = Block::default()
        .title(" デバッグ図鑑 ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    let body = Paragraph::new("（未実装）").block(block);
    f.render_widget(body, chunks[0]);

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let footer = Paragraph::new(Line::from("  [Q/Esc] 戻る")).block(footer_block);
    f.render_widget(footer, chunks[1]);
}

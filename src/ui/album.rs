use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::save::schema::SaveData;
use crate::game::evolution;

pub struct AlbumState {
    pub scroll: usize,
}

impl AlbumState {
    pub fn new() -> Self {
        Self { scroll: 0 }
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self, max_items: usize, visible_lines: usize) {
        if self.scroll + visible_lines < max_items {
            self.scroll += 1;
        }
    }
}

pub fn render_album(f: &mut Frame, save_data: &SaveData, album_state: &AlbumState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // header
            Constraint::Min(4),    // body
            Constraint::Length(3), // footer
        ])
        .split(f.area());

    // --- Header ---
    let all_species = evolution::all_species_names();
    let total_species = all_species.len();

    // Count encountered species (current pet + album)
    let mut encountered: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Some(ref pet) = save_data.pet {
        for species in &pet.evolution_line {
            encountered.insert(species.clone());
        }
        encountered.insert(pet.species.clone());
    }
    for entry in &save_data.album {
        for species in &entry.evolution_line {
            encountered.insert(species.clone());
        }
    }
    // Exclude egg
    encountered.remove("たまご");

    let encountered_count = encountered.len();

    let header_text = format!("  📖 図鑑                              {} / {}+", encountered_count, total_species);
    let header_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));

    f.render_widget(header_block, chunks[0]);

    let header_inner = Rect {
        x: chunks[0].x,
        y: chunks[0].y + 1,
        width: chunks[0].width,
        height: chunks[0].height.saturating_sub(2),
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            header_text,
            Style::default().add_modifier(Modifier::BOLD),
        ))),
        header_inner,
    );

    // --- Body: species list ---
    let body_height = chunks[1].height as usize;

    // Build display lines
    let mut lines: Vec<Line> = Vec::new();

    for (i, species_name) in all_species.iter().enumerate() {
        let num = format!("#{:03}", i + 1);

        if encountered.contains(*species_name) {
            // Find album entry for this species (most recent)
            let album_entry = save_data.album.iter().rev()
                .find(|e| e.evolution_line.contains(&species_name.to_string()) || e.species == *species_name);

            // Check if currently alive
            let is_current = save_data.pet.as_ref()
                .map(|p| p.species == *species_name || p.evolution_line.contains(&species_name.to_string()))
                .unwrap_or(false);

            if let Some(entry) = album_entry {
                let info = format!(
                    "  {} {}  ✓ {}日間生きた : {:.0}kg（{}）",
                    num, species_name, entry.days_lived, entry.weight_kg, entry.weight_label
                );
                lines.push(Line::from(Span::styled(
                    info,
                    Style::default().fg(Color::White),
                )));
            } else if is_current {
                let info = format!("  {} {}  ★ 育成中！", num, species_name);
                lines.push(Line::from(Span::styled(
                    info,
                    Style::default().fg(Color::Green),
                )));
            } else {
                let info = format!("  {} {}  ✓", num, species_name);
                lines.push(Line::from(Span::styled(
                    info,
                    Style::default().fg(Color::White),
                )));
            }
        } else {
            let info = format!("  {} ？？？", num);
            lines.push(Line::from(Span::styled(
                info,
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    // Add mystery line
    lines.push(Line::from(Span::styled(
        "  #??? まだ見ぬモンスターがいるようだ…",
        Style::default().fg(Color::DarkGray),
    )));

    // Apply scroll
    let visible_lines: Vec<Line> = lines
        .into_iter()
        .skip(album_state.scroll)
        .take(body_height)
        .collect();

    let body = Paragraph::new(visible_lines);
    f.render_widget(body, chunks[1]);

    // --- Footer ---
    let footer_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));

    f.render_widget(footer_block, chunks[2]);

    let footer_inner = Rect {
        x: chunks[2].x,
        y: chunks[2].y + 1,
        width: chunks[2].width,
        height: chunks[2].height.saturating_sub(2),
    };
    let footer = Paragraph::new(Line::from("  [↑↓] スクロール                    [Q] 戻る"));
    f.render_widget(footer, footer_inner);
}

/// Total number of species displayed in the album
pub fn total_species_count() -> usize {
    evolution::all_species_names().len()
}

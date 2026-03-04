use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::game::actions::Action;
use crate::game::evolution;
use crate::game::pet::MoodLevel;
use crate::ui::ascii_art;

const ALL_ACTIONS: [Action; 4] = [Action::Talk, Action::Play, Action::Train, Action::Relax];

/// Which sub-view the debug album is in.
#[derive(Clone)]
pub enum DebugAlbumView {
    /// Species list (like the regular album, but all visible).
    List,
    /// Viewing idle animation for selected species.
    Idle,
    /// Viewing an action animation for selected species.
    Action { index: usize },
}

pub struct DebugAlbumState {
    pub view: DebugAlbumView,
    pub cursor: usize,
    pub scroll: usize,
}

impl DebugAlbumState {
    pub fn new() -> Self {
        Self {
            view: DebugAlbumView::List,
            cursor: 0,
            scroll: 0,
        }
    }

    pub fn species_count() -> usize {
        evolution::all_species_names().len()
    }

    pub fn selected_species(&self) -> &'static str {
        let names = evolution::all_species_names();
        names[self.cursor]
    }

    pub fn cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            if self.cursor < self.scroll {
                self.scroll = self.cursor;
            }
        }
    }

    pub fn cursor_down(&mut self, visible: usize) {
        let max = Self::species_count().saturating_sub(1);
        if self.cursor < max {
            self.cursor += 1;
            if self.cursor >= self.scroll + visible {
                self.scroll = self.cursor - visible + 1;
            }
        }
    }

    pub fn next_action(&mut self) {
        if let DebugAlbumView::Action { ref mut index } = self.view {
            *index = (*index + 1) % ALL_ACTIONS.len();
        }
    }

    pub fn prev_action(&mut self) {
        if let DebugAlbumView::Action { ref mut index } = self.view {
            *index = (*index + ALL_ACTIONS.len() - 1) % ALL_ACTIONS.len();
        }
    }
}

// ─── Rendering ───────────────────────────────────────────

pub fn render_debug_album(f: &mut Frame, state: &DebugAlbumState, animation_frame: usize) {
    match &state.view {
        DebugAlbumView::List => render_list(f, state),
        DebugAlbumView::Idle => render_idle(f, state, animation_frame),
        DebugAlbumView::Action { index } => render_action(f, state, *index, animation_frame),
    }
}

// ─── List view ───────────────────────────────────────────

fn render_list(f: &mut Frame, state: &DebugAlbumState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(4),   // body
            Constraint::Length(3), // footer
        ])
        .split(f.area());

    // --- Header ---
    let all_species = evolution::all_species_names();
    let total = all_species.len();

    let header_text = format!(
        "  🔧 デバッグ図鑑                       {} 種",
        total
    );
    let header_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::Yellow));
    f.render_widget(header_block, chunks[0]);

    let header_inner = inner_area(chunks[0], 0, 1);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            header_text,
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow),
        ))),
        header_inner,
    );

    // --- Body: species list with cursor ---
    let body_height = chunks[1].height as usize;
    let mut lines: Vec<Line> = Vec::new();

    for (i, species_name) in all_species.iter().enumerate().skip(state.scroll).take(body_height) {
        let num = format!("#{:03}", i + 1);
        let stage = evolution::get_stage(species_name).unwrap_or(1);

        let is_selected = i == state.cursor;
        let marker = if is_selected { "▶" } else { " " };

        let text = format!(" {} {} {} [S{}]", marker, num, species_name, stage);

        let style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        lines.push(Line::from(Span::styled(text, style)));
    }

    let body = Paragraph::new(lines);
    f.render_widget(body, chunks[1]);

    // --- Footer ---
    let footer_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer_block, chunks[2]);

    let footer_inner = inner_area(chunks[2], 0, 1);
    let footer = Paragraph::new(Line::from(
        "  [↑↓] 選択  [Enter] プレビュー          [Q] 戻る",
    ));
    f.render_widget(footer, footer_inner);
}

// ─── Idle animation view ─────────────────────────────────

fn render_idle(f: &mut Frame, state: &DebugAlbumState, animation_frame: usize) {
    let species = state.selected_species();
    let stage = evolution::get_stage(species).unwrap_or(1);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(8),   // body
            Constraint::Length(3), // footer
        ])
        .split(f.area());

    // --- Header ---
    let header_text = format!(
        "  {} [S{}] — 待機アニメーション",
        species, stage
    );
    let header_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::Yellow));
    f.render_widget(header_block, chunks[0]);

    let header_inner = inner_area(chunks[0], 0, 1);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            header_text,
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow),
        ))),
        header_inner,
    );

    // --- Body: show all 3 moods side by side vertically ---
    let moods = [
        (MoodLevel::High, "High (きげん◎)"),
        (MoodLevel::Normal, "Normal (ふつう)"),
        (MoodLevel::Low, "Low (きげん✗)"),
    ];

    let mut body_lines: Vec<Line> = Vec::new();

    for (mood, label) in &moods {
        body_lines.push(Line::from(Span::styled(
            format!("  ── {} ──", label),
            Style::default().fg(Color::Cyan),
        )));

        let art = ascii_art::get_art(species, *mood, animation_frame);
        for art_line in art {
            body_lines.push(Line::from(format!("        {}", art_line)));
        }
        body_lines.push(Line::from(""));
    }

    let body = Paragraph::new(body_lines).alignment(Alignment::Left);
    f.render_widget(body, chunks[1]);

    // --- Footer ---
    let footer_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer_block, chunks[2]);

    let footer_inner = inner_area(chunks[2], 0, 1);
    let footer = Paragraph::new(Line::from(
        "  [A] アクション表示  [Esc] 一覧へ戻る",
    ));
    f.render_widget(footer, footer_inner);
}

// ─── Action animation view ───────────────────────────────

fn render_action(f: &mut Frame, state: &DebugAlbumState, action_index: usize, animation_frame: usize) {
    let species = state.selected_species();
    let stage = evolution::get_stage(species).unwrap_or(1);
    let action = ALL_ACTIONS[action_index];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(8),   // body
            Constraint::Length(3), // footer
        ])
        .split(f.area());

    // --- Header ---
    let header_text = format!(
        "  {} [S{}] — {} アニメーション",
        species, stage, action.label()
    );
    let header_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::Yellow));
    f.render_widget(header_block, chunks[0]);

    let header_inner = inner_area(chunks[0], 0, 1);
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            header_text,
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow),
        ))),
        header_inner,
    );

    // --- Body: action animation + effect ---
    let art = ascii_art::get_action_art(species, action, animation_frame);
    let effect = ascii_art::get_action_effect(action, animation_frame);

    let mut body_lines: Vec<Line> = Vec::new();
    body_lines.push(Line::from(""));

    // Show which action with navigation hint
    let nav_line: Vec<Span> = vec![
        Span::styled("  ◀ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("[{}] {}", action_key(action), action.label()),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ▶", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("  ({}/{})", action_index + 1, ALL_ACTIONS.len()),
            Style::default().fg(Color::DarkGray),
        ),
    ];
    body_lines.push(Line::from(nav_line));
    body_lines.push(Line::from(""));

    for art_line in art {
        body_lines.push(Line::from(format!("        {}", art_line)));
    }

    body_lines.push(Line::from(""));
    body_lines.push(Line::from(Span::styled(
        format!("  {}", effect),
        Style::default().fg(Color::Cyan),
    )));

    let body = Paragraph::new(body_lines).alignment(Alignment::Left);
    f.render_widget(body, chunks[1]);

    // --- Footer ---
    let footer_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer_block, chunks[2]);

    let footer_inner = inner_area(chunks[2], 0, 1);
    let footer = Paragraph::new(Line::from(
        "  [←→] アクション切替  [I] 待機へ  [Esc] 一覧へ戻る",
    ));
    f.render_widget(footer, footer_inner);
}

fn action_key(action: Action) -> &'static str {
    match action {
        Action::Talk => "T",
        Action::Play => "P",
        Action::Train => "R",
        Action::Relax => "E",
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

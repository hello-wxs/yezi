// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Entries page

/// Render the entries page
pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &mut crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    let cfg = crate::get_config();
    // Render entries list
    let list = ratatui::widgets::List::new(make_lines(
        app_state
            .current_book()
            .expect("Get current book")
            .get_entries(),
    ))
    .style(cfg.theme.fg.common)
    .highlight_style(cfg.theme.fg.important)
    .scroll_padding(2);

    f.render_stateful_widget(list, area, &mut app_state.selected.entry);
}
/// Handle key events for the entries page
pub(crate) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut crate::state::AppState,
) {
    use crossterm::event::{KeyCode, KeyEventKind};
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Char('g') => {
                if app_state.selected.entry.selected().is_none() {
                    app_state.selected.entry.select(Some(0));
                }
                app_state.current_view.set_learn();
            }
            KeyCode::Up | KeyCode::Char('j') => app_state.selected.entry.select_previous(),
            KeyCode::Down | KeyCode::Char('k') => app_state.selected.entry.select_next(),
            KeyCode::Left | KeyCode::Char('h') => {
                app_state.selected.entry.select(None);
                app_state.current_view.set_books();
            }
            _ => {}
        }
    }
}

fn make_lines(entries: &[yezi_data::Entry]) -> Vec<String> {
    entries
        .iter()
        .enumerate()
        .map(|(idx, entry)| idx.to_string() + ". " + entry.get_key() + " -> " + entry.get_value())
        .collect()
}

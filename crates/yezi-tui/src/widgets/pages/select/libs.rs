// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Lib page

/// Render the lib page
pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &mut crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    let cfg = crate::get_config();
    // Render lib list
    let list = ratatui::widgets::List::new(make_lines(&app_state.data))
        .highlight_style(cfg.theme.fg.important)
        .style(cfg.theme.fg.common)
        .scroll_padding(2);

    f.render_stateful_widget(list, area, &mut app_state.selected.lib);
}

/// Handle key events for the lib page
pub(crate) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut crate::state::AppState,
) {
    use crossterm::event::{KeyCode, KeyEventKind};
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('j') => app_state.selected.lib.select_previous(),
            KeyCode::Down | KeyCode::Char('k') => app_state.selected.lib.select_next(),
            KeyCode::Right | KeyCode::Char('l') if app_state.selected.lib.selected().is_some() => {
                app_state.current_view.set_books();
            }
            _ => {}
        }
    }
}

fn make_lines(libs: &[yezi_data::Lib]) -> Vec<String> {
    libs.iter()
        .enumerate()
        .map(|(idx, lib)| idx.to_string() + ". " + lib.get_name() + " - " + lib.get_description())
        .collect()
}

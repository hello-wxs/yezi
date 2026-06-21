// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Book page

/// Render book page
pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &mut crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    let cfg = crate::get_config();
    // Render books list
    let list = ratatui::widgets::List::new(make_lines(
        app_state
            .current_lib()
            .expect("Get current lib.")
            .get_books(),
    ))
    .style(cfg.theme.fg.common)
    .highlight_style(cfg.theme.fg.important)
    .scroll_padding(2);

    f.render_stateful_widget(list, area, &mut app_state.selected.book);
}
/// Handle book page key events
pub(crate) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut crate::state::AppState,
) {
    use crossterm::event::{KeyCode, KeyEventKind};
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Up | KeyCode::Char('j') => app_state.selected.book.select_previous(),
            KeyCode::Down | KeyCode::Char('k') => app_state.selected.book.select_next(),
            KeyCode::Left | KeyCode::Char('h') => {
                app_state.selected.book.select(None);
                app_state.current_view.set_libs();
            }
            KeyCode::Right | KeyCode::Char('l')
                if app_state.selected.book.selected().is_some() => {
                    app_state.current_view.set_entries();
                }
            KeyCode::Char('g')
                // Learn the book
                if app_state.selected.book.selected().is_some() => {
                    app_state.selected.entry.select(Some(0));
                    app_state.current_view.set_learn();
                }
            _ => {}
        }
    }
}

fn make_lines(books: &[yezi_data::Book]) -> Vec<String> {
    books
        .iter()
        .enumerate()
        .map(|(idx, book)| {
            idx.to_string() + ". " + book.get_name() + " - " + book.get_description()
        })
        .collect()
}

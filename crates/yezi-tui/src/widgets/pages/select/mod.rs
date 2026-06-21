// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod books;
pub(crate) mod entries;
pub(crate) mod libs;

pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &mut crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    if let crate::state::AppView::Select(ref select) = app_state.current_view {
        match select {
            crate::state::Select::Libs => libs::render(f, app_state, area),
            crate::state::Select::Books => books::render(f, app_state, area),
            crate::state::Select::Entries => entries::render(f, app_state, area),
        }
    }
}

pub(crate) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut crate::state::AppState,
) {
    if let crate::state::AppView::Select(ref select) = app_state.current_view {
        match select {
            crate::state::Select::Libs => libs::handle_key(key_event, app_state),
            crate::state::Select::Books => books::handle_key(key_event, app_state),
            crate::state::Select::Entries => entries::handle_key(key_event, app_state),
        }
    }
}

// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod input;
pub(crate) mod state;

pub(crate) enum FeedBack {
    Input(input::FeedBack),
}

pub(super) fn render(
    f: &mut ratatui::Frame,
    app_state: &crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    let [state_area, input_area] = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Min(1),
        ratatui::layout::Constraint::Length(3),
    ])
    .areas(area);
    // Render
    state::render(f, app_state, state_area);
    input::render(f, app_state, input_area);
}

pub(super) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &crate::state::AppState,
) -> FeedBack {
    FeedBack::Input(input::handle_key(key_event, app_state))
}

pub(super) fn update(app_state: &mut crate::state::AppState, feedback: FeedBack) {
    match feedback {
        FeedBack::Input(input_feedback) => input::update(app_state, input_feedback),
    }
}

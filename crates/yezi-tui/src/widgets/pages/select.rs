// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) enum FeedBack {
    None,
}

pub(super) fn render(
    _f: &mut ratatui::Frame,
    _app_state: &crate::app::State,
    _area: ratatui::layout::Rect,
) {
}

pub(super) fn handle_key(
    _key_event: crossterm::event::KeyEvent,
    _app_state: &crate::app::State,
) -> FeedBack {
    FeedBack::None
}

pub(super) fn update(_app_state: &mut crate::app::State, feedback: FeedBack) {
    match feedback {
        FeedBack::None => {}
    }
}

// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(super) enum FeedBack {
    None,
}

pub(crate) fn render(
    _f: &mut ratatui::Frame,
    _app_state: &crate::state::AppState,
    _area: ratatui::layout::Rect,
) {
}

pub(crate) fn handle_key(
    _key_event: crossterm::event::KeyEvent,
    _app_state: &crate::state::AppState,
) -> FeedBack {
    FeedBack::None
}

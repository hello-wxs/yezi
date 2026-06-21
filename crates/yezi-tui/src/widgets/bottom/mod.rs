// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod buddy;
pub(crate) mod info;

pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &mut crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    let cfg = crate::get_config();
    let [info_area, buddy_area] = ratatui::layout::Layout::horizontal([
        ratatui::layout::Constraint::Min(0),
        if cfg.user.name.is_some() && cfg.buddy.show {
            ratatui::layout::Constraint::Length(8)
        } else {
            ratatui::layout::Constraint::Length(0)
        },
    ])
    .areas(area);
    // Render buddy area
    buddy::render(f, app_state, buddy_area);
    info::render(f, app_state, info_area);
}
pub(crate) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut crate::state::AppState,
) {
    info::handle_key(key_event, app_state);
}

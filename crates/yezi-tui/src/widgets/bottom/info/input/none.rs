// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) fn render(
    f: &mut ratatui::Frame,
    _app_state: &mut crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    use ratatui::style::Stylize;

    let cfg = crate::get_config();
    f.render_widget(
        ratatui::widgets::Paragraph::new(" Press `:` to select command mode").fg(cfg.theme.fg.less),
        area,
    );
}
pub(crate) fn handle_key(
    _key_event: crossterm::event::KeyEvent,
    _app_state: &mut crate::state::AppState,
) {
}

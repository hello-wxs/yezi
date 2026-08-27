// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Entries page

pub(super) fn render(
    f: &mut ratatui::Frame,
    app_state: &crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    use ratatui::style::Stylize;

    let image = ratatui::text::Text::from(app_state.buddy.get_image());
    let par = ratatui::widgets::Paragraph::new(image).fg(app_state.buddy.get_color());
    f.render_widget(par, area);
}

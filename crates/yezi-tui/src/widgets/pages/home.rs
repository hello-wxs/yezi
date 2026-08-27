// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Home page

#[derive(Debug, Default)]
pub(crate) struct State {}

pub(crate) enum FeedBack {
    None,
}

/// Render the home page
pub(super) fn render(
    f: &mut ratatui::Frame,
    _app_state: &crate::app::State,
    area: ratatui::layout::Rect,
) {
    let cfg = crate::get_config();
    // Set Text
    let text = ratatui::text::Text::from(vec![
        ratatui::text::Line::from(env!("CARGO_PKG_NAME")),
        ratatui::text::Line::from(concat!("version: ", env!("CARGO_PKG_VERSION"))),
        ratatui::text::Line::from(env!("CARGO_PKG_LICENSE")),
        ratatui::text::Line::from(concat!("By ", env!("CARGO_PKG_AUTHORS"), " at all.")),
        ratatui::text::Line::from(env!("CARGO_PKG_DESCRIPTION")),
        ratatui::text::Line::default(),
        ratatui::text::Line::from("type  :q <Enter>               to exit      "),
        ratatui::text::Line::from("type  :o path <Enter>          to open a lib"),
    ])
    .style(cfg.theme.fg.common);
    // Render text
    let [_, main_area, _] = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Min(0),
        ratatui::layout::Constraint::Length(text.iter().len().try_into().unwrap()), // Never panic
        ratatui::layout::Constraint::Min(0),
    ])
    .areas(area);
    let paragraph =
        ratatui::widgets::Paragraph::new(text).alignment(ratatui::layout::Alignment::Center);
    f.render_widget(paragraph, main_area);
}
/// Handle key events for the home page
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

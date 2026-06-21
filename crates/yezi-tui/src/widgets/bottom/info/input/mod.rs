// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::state::CurrentInput;

pub(crate) mod cmd;
pub(crate) mod none;

pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &mut crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    let cfg = crate::get_config();
    let border = ratatui::widgets::Block::bordered()
        .title(ratatui::text::Line::from(match app_state.current_input {
            CurrentInput::None => "input",
            CurrentInput::Cmd(_) => "command",
            CurrentInput::Search(_) => "search",
        }))
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(cfg.theme.fg.common);
    let inside_are = border.inner(area);
    f.render_widget(border, area);
    match app_state.current_input {
        CurrentInput::None => none::render(f, app_state, inside_are),
        CurrentInput::Cmd(_) => cmd::render(f, app_state, inside_are),
        CurrentInput::Search(_) => {}
    }
}

pub(crate) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut crate::state::AppState,
) {
    match app_state.current_input {
        CurrentInput::None => none::handle_key(key_event, app_state),
        CurrentInput::Cmd(_) => cmd::handle_key(key_event, app_state),
        CurrentInput::Search(_) => {}
    }
}

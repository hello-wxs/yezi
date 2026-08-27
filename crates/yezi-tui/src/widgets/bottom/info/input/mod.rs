// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod cmd;
pub(crate) mod none;
pub(super) mod search;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Input {
    None,
    Cmd(cmd::State),
    #[allow(dead_code)]
    Search(search::SearchState),
}

pub(crate) enum FeedBack {
    None,
    Cmd(cmd::FeedBack),
    Search,
}

pub(super) fn render(
    f: &mut ratatui::Frame,
    app_state: &crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    let cfg = crate::get_config();
    let border = ratatui::widgets::Block::bordered()
        .title(ratatui::text::Line::from(match app_state.current_input {
            Input::None => "input",
            Input::Cmd(_) => "command",
            Input::Search(_) => "search",
        }))
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(cfg.theme.fg.common);
    let inside_area = border.inner(area);
    f.render_widget(border, area);
    match app_state.current_input {
        Input::None => none::render(f, inside_area),
        Input::Cmd(_) => cmd::render(f, app_state, inside_area),
        Input::Search(_) => {}
    }
}

pub(super) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &crate::state::AppState,
) -> FeedBack {
    match app_state.current_input {
        Input::None => FeedBack::None,
        Input::Cmd(_) => FeedBack::Cmd(cmd::handle_key(key_event, app_state)),
        Input::Search(_) => FeedBack::Search,
    }
}

pub(super) fn update(app_state: &mut crate::state::AppState, feedback: FeedBack) {
    match feedback {
        FeedBack::None => {}
        FeedBack::Cmd(cmd_feedback) => cmd::update(app_state, cmd_feedback),
        FeedBack::Search => {}
    }
}

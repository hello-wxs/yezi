// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Pages module

pub(crate) mod bottom;
pub(crate) mod pages;

pub(super) enum FeedBack {
    OpenCmd,
    Bottom(bottom::FeedBack),
    Pages(pages::FeedBack),
}

pub(super) fn render(
    f: &mut ratatui::Frame,
    app_state: &crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    use ratatui::prelude::Stylize;

    let cfg = crate::get_config();
    let background = ratatui::widgets::Block::default().bg(cfg.theme.bg);
    f.render_widget(background, area);

    // Split Screen
    let [page_area, bottom_area] = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Min(0),
        ratatui::layout::Constraint::Length(4),
    ])
    .areas(area);
    // Render pages
    pages::render(f, app_state, page_area);
    bottom::render(f, app_state, bottom_area);
}

pub(super) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &crate::state::AppState,
) -> FeedBack {
    if key_event.kind == crossterm::event::KeyEventKind::Press
        && key_event.code == crossterm::event::KeyCode::Char(':')
    {
        return FeedBack::OpenCmd;
    }
    if crate::state::CurrentInput::None == app_state.current_input {
        FeedBack::Pages(pages::handle_key(key_event, app_state))
    } else {
        FeedBack::Bottom(bottom::handle_key(key_event, app_state))
    }
}

pub(super) fn update(app_state: &mut crate::state::AppState, feedback: FeedBack) {
    match feedback {
        FeedBack::OpenCmd => {
            app_state.current_input =
                crate::state::CurrentInput::Cmd(crate::state::CmdState::Input(String::new()));
        }
        FeedBack::Bottom(bottom_feedback) => {
            bottom::update(app_state, bottom_feedback);
        }
        FeedBack::Pages(pages_feedback) => {
            pages::update(app_state, pages_feedback);
        }
    }
}

// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod home;
pub(crate) mod learn;
pub(crate) mod select;

pub(crate) enum FeedBack {
    Home(home::FeedBack),
    Learn(learn::FeedBack),
    Select(select::FeedBack),
}

pub(super) fn render(
    f: &mut ratatui::Frame,
    app_state: &crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    let cfg = crate::get_config();
    let border = ratatui::widgets::Block::default()
        .title(match app_state.current_view {
            crate::state::AppView::Home => "home",
            crate::state::AppView::Select => "select",
            crate::state::AppView::Learn(_) => "learn",
        })
        .borders(ratatui::widgets::Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(ratatui::style::Style::default().fg(cfg.theme.fg.common));
    f.render_widget(&border, area);
    let inner_area = border.inner(area);
    match app_state.current_view {
        crate::state::AppView::Home => home::render(f, app_state, inner_area),
        crate::state::AppView::Select => select::render(f, app_state, inner_area),
        crate::state::AppView::Learn(_) => learn::render(f, app_state, inner_area),
    }
}

pub(super) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &crate::state::AppState,
) -> FeedBack {
    match app_state.current_view {
        crate::state::AppView::Home => FeedBack::Home(home::handle_key(key_event, app_state)),
        crate::state::AppView::Select => FeedBack::Select(select::handle_key(key_event, app_state)),
        crate::state::AppView::Learn(_) => FeedBack::Learn(learn::handle_key(key_event, app_state)),
    }
}

pub(super) fn update(app_state: &mut crate::state::AppState, feedback: FeedBack) {
    match feedback {
        FeedBack::Home(home_feedback) => {
            home::update(app_state, home_feedback);
        }
        FeedBack::Select(select_feedback) => {
            select::update(app_state, select_feedback);
        }
        FeedBack::Learn(learn_feedback) => {
            learn::update(app_state, learn_feedback);
        }
    }
}

// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) mod home;
pub(crate) mod learn;
pub(crate) mod select;

pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &mut crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    let cfg = crate::get_config();
    let border = ratatui::widgets::Block::default()
        .title(match app_state.current_view {
            crate::state::AppView::Home => "Home",
            crate::state::AppView::Select(ref select) => match select {
                crate::state::Select::Libs => "Libs",
                crate::state::Select::Books => "Books",
                crate::state::Select::Entries => "Entries",
            },
            crate::state::AppView::Learn(_) => "Learn",
        })
        .borders(ratatui::widgets::Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(ratatui::style::Style::default().fg(cfg.theme.fg.common));
    f.render_widget(&border, area);
    let inner_area = border.inner(area);
    match app_state.current_view {
        crate::state::AppView::Home => home::render(f, app_state, inner_area),
        crate::state::AppView::Select(_) => select::render(f, app_state, inner_area),
        crate::state::AppView::Learn(_) => learn::render(f, app_state, inner_area),
    }
}

pub(crate) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut crate::state::AppState,
) {
    match app_state.current_view {
        crate::state::AppView::Home => home::handle_key(key_event, app_state),
        crate::state::AppView::Select(_) => select::handle_key(key_event, app_state),
        crate::state::AppView::Learn(_) => learn::handle_key(key_event, app_state),
    }
}

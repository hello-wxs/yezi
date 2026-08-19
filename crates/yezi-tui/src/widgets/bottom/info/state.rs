// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! State line

pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &mut crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    let cfg = crate::get_config();

    let show_saying = cfg.buddy.show
        && cfg.user.name.is_some()
        && area.width as usize
            > get_location(app_state).len() + app_state.buddy.get_saying().len() + 2;

    let [location_area, buddy_saying_area] = ratatui::layout::Layout::horizontal([
        ratatui::layout::Constraint::Min(0),
        if show_saying {
            ratatui::layout::Constraint::Length(app_state.buddy.get_saying().len() as u16)
        } else {
            ratatui::layout::Constraint::Length(0)
        },
    ])
    .areas(area);

    f.render_widget(
        ratatui::widgets::Paragraph::new(" ".to_string() + &get_location(app_state))
            .left_aligned()
            .style(cfg.theme.fg.common),
        location_area,
    );
    f.render_widget(
        ratatui::widgets::Paragraph::new(app_state.buddy.get_saying() + " ")
            .right_aligned()
            .style(cfg.theme.fg.less),
        buddy_saying_area,
    );
}

fn get_location(app_state: &crate::state::AppState) -> String {
    let mut result = "root".to_string();
    if let Some(lib) = app_state.current_lib() {
        result += " / ";
        result.push_str(&lib.get_name().clone());
        if let Some(book) = app_state.current_book() {
            result += " / ";
            result.push_str(&book.get_name().clone());
            if let Some(entry) = app_state.current_entry() {
                result += " / ";
                if app_state.current_view.is_entries() {
                    result.push_str(&format!(
                        "{} -> {}",
                        entry.get("key").unwrap_or_default().to_owned(),
                        entry.get("value").unwrap_or_default().to_owned()
                    ));
                } else {
                    result.push_str(entry.get("key").unwrap_or_default());
                }
            }
        }
    }
    result
}

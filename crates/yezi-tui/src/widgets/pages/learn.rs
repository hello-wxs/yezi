// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Learn page

/// Render learn page
pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &mut crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    use ratatui::style::Stylize;

    let cfg = crate::get_config();

    let area = area.inner(ratatui::layout::Margin::new(5, 0));

    let learn_state = &app_state
        .current_view
        .learn_state()
        .expect("Page view wrong!");
    let now_entry = app_state.current_entry().unwrap();
    // Learn contect
    let text = ratatui::text::Text::from(vec![
        // Key
        ratatui::text::Line::from(now_entry.get_key().clone())
            .alignment(ratatui::layout::Alignment::Center)
            .fg(cfg.theme.fg.common),
        // Value
        ratatui::text::Line::from(current_words(now_entry.get_value(), learn_state.now_show))
            .alignment(ratatui::layout::Alignment::Center)
            .fg(cfg.theme.fg.important),
        // Tip
        ratatui::text::Line::from(if learn_state.show_tip {
            now_entry.get_tip().clone()
        } else {
            String::from("--hidden--")
        })
        .alignment(ratatui::layout::Alignment::Center)
        .fg(cfg.theme.fg.less),
    ]);
    // Split area
    let [_, learn_area, _] = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Min(0),
        ratatui::layout::Constraint::Length(text.iter().len().try_into().unwrap()),
        ratatui::layout::Constraint::Min(0),
    ])
    .areas(area);
    // Render
    f.render_widget(
        ratatui::widgets::Paragraph::new(text).wrap(ratatui::widgets::Wrap { trim: true }),
        learn_area,
    );
}
/// Handle learn page key events
pub(crate) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &mut crate::state::AppState,
) {
    use crossterm::event::{KeyCode, KeyEventKind};
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Char('q') => {
                app_state.current_view.set_books();
            }
            KeyCode::Up | KeyCode::Char('j') => {
                app_state.selected.entry.select_previous();
                app_state.learn_state_mut().unwrap().now_show = 0;
                app_state.learn_state_mut().unwrap().show_tip = false;
            }
            KeyCode::Down | KeyCode::Char('k') => {
                let entries_len = app_state.current_book().unwrap().get_entries().len();
                if let Some(selected) = app_state.selected.entry.selected()
                    && selected + 1 < entries_len
                {
                    app_state.selected.entry.select_next();
                    app_state.learn_state_mut().unwrap().now_show = 0;
                    app_state.learn_state_mut().unwrap().show_tip = false;
                }
            }
            KeyCode::Left | KeyCode::Char('h') if app_state.learn_state().unwrap().now_show > 0 => {
                app_state.learn_state_mut().unwrap().now_show -= 1;
            }
            KeyCode::Right | KeyCode::Char('l')
                if app_state.learn_state_mut().unwrap().now_show
                    < app_state
                        .current_entry()
                        .unwrap()
                        .get_value()
                        .chars()
                        .count() =>
            {
                app_state.learn_state_mut().unwrap().now_show += 1;
            }
            KeyCode::Char('t') => {
                app_state.learn_state_mut().unwrap().show_tip =
                    !app_state.learn_state_mut().unwrap().show_tip
            }
            _ => {}
        }
    }
}
fn current_words(full: &str, len: usize) -> String {
    use unicode_width::UnicodeWidthStr;

    let show_part = full.chars().take(len).collect::<String>();
    show_part.clone()
        + &"_".repeat(full.width() - full.chars().take(len).collect::<String>().width())
}

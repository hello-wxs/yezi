// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Learn page

pub(crate) enum FeedBack {
    None,
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct State {
    /// Currently showing entry
    pub(crate) now_show: usize,
    /// Whether to show tip
    pub(crate) show_tip: bool,
}

/// Render learn page
pub(crate) fn render(
    _f: &mut ratatui::Frame,
    _app_state: &crate::state::AppState,
    _area: ratatui::layout::Rect,
) {
    /*
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
        ratatui::text::Line::from(now_entry.get("key").unwrap_or_default())
            .alignment(ratatui::layout::Alignment::Center)
            .fg(cfg.theme.fg.common),
        // Value
        ratatui::text::Line::from(current_words(
            now_entry.get("value").unwrap_or_default(),
            learn_state.now_show,
        ))
        .alignment(ratatui::layout::Alignment::Center)
        .fg(cfg.theme.fg.important),
        // Tip
        ratatui::text::Line::from(if learn_state.show_tip {
            now_entry.get("tip").unwrap_or_default().to_owned()
        } else {
            String::from("--hidden--")
        })
        .alignment(ratatui::layout::Alignment::Center)
        .fg(cfg.theme.fg.less),
    ]);
    let height = text
        .iter()
        .map(|line| {
            (line.width() as u16)
                .checked_div_ceil(area.width)
                .unwrap_or_default()
        })
        .sum::<u16>();
    let par = ratatui::widgets::Paragraph::new(text)
        .alignment(ratatui::layout::Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    // Split area
    let [_, learn_area, _] = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Min(0),
        ratatui::layout::Constraint::Length(height),
        ratatui::layout::Constraint::Min(0),
    ])
    .areas(area);
    // Render
    f.render_widget(par, learn_area);
    */
}
/// Handle learn page key events
pub(super) fn handle_key(
    _key_event: crossterm::event::KeyEvent,
    _app_state: &crate::state::AppState,
) -> FeedBack {
    FeedBack::None
    /*
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
                        .get("value")
                        .unwrap_or_default()
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
    */
}
fn _current_words(full: &str, len: usize) -> String {
    use unicode_width::UnicodeWidthStr;

    let show_part = full.chars().take(len).collect::<String>();
    show_part.clone()
        + &"_".repeat(full.width() - full.chars().take(len).collect::<String>().width())
}

trait _CheckedDivCeil {
    fn checked_div_ceil(self, other: Self) -> Option<Self>
    where
        Self: Sized;
}

impl _CheckedDivCeil for u16 {
    fn checked_div_ceil(self, other: Self) -> Option<u16> {
        if other == 0 {
            return None;
        }
        Some(self.div_ceil(other))
    }
}

pub(super) fn update(_app_state: &mut crate::state::AppState, feedback: FeedBack) {
    match feedback {
        FeedBack::None => {}
    }
}

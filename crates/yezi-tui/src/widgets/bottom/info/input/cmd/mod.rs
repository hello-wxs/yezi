// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Command line

mod run;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum State {
    /// Command line is inputting
    Input(String),
    /// Command line execution succeeded
    Success,
    /// Command line execution error
    Error(String),
    /// With doc
    Doc(String),
}

impl Default for State {
    fn default() -> Self {
        Self::Input(String::new())
    }
}

impl State {
    pub(crate) fn get_input(&self) -> Option<&str> {
        match self {
            Self::Input(input) => Some(input),
            _ => None,
        }
    }
}

pub(crate) enum FeedBack {
    None,
    Input(char),
    Delete,
    Quit,
    Run(run::FeedBack),
}

/// Render command line
pub(super) fn render(
    f: &mut ratatui::Frame,
    app_state: &crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    use ratatui::style::Stylize;

    let cfg = crate::get_config();
    if let crate::widgets::bottom::info::input::Input::Cmd(cmd_state) = &app_state.current_input {
        // Select background color based on the command type
        let (color, content) = match cmd_state {
            State::Input(content) => (cfg.theme.fg.important, " :".to_owned() + content),
            State::Success => (
                cfg.theme.fg.less,
                " Command executed successfully".to_owned(),
            ),
            State::Error(content) => (cfg.theme.fg.err, " ".to_owned() + content),
            State::Doc(content) => (cfg.theme.fg.common, " ".to_owned() + content),
        };
        f.render_widget(ratatui::widgets::Paragraph::new(content).fg(color), area);
    }
}

/// Handle command line key events
pub(super) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &crate::state::AppState,
) -> FeedBack {
    use crossterm::event::{KeyCode, KeyEventKind};

    // Only handle key when you press expect release
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Char(key) => FeedBack::Input(key),
            KeyCode::Backspace => FeedBack::Delete,
            KeyCode::Enter => FeedBack::Run(run::try_run_cmd(app_state)),
            KeyCode::Esc => FeedBack::Quit,
            _ => FeedBack::None,
        }
    } else {
        FeedBack::None
    }
}

pub(super) fn update(app_state: &mut crate::state::AppState, feedback: FeedBack) {
    match feedback {
        FeedBack::None => {}
        FeedBack::Input(input) => {
            let crate::widgets::bottom::info::input::Input::Cmd(State::Input(ref mut content)) =
                app_state.current_input
            else {
                unreachable!()
            };
            content.push(input);
        }
        FeedBack::Delete => {
            let crate::widgets::bottom::info::input::Input::Cmd(State::Input(ref mut content)) =
                app_state.current_input
            else {
                unreachable!()
            };
            if content.pop().is_none() {
                app_state.current_input = crate::widgets::bottom::info::input::Input::None;
            }
        }
        FeedBack::Run(run_feedback) => run::update(app_state, run_feedback),
        FeedBack::Quit => {}
    }
}

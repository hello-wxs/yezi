// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Command line

mod run;

pub(super) enum FeedBack {
    None,
    Input(char),
    Delete,
    Quit,
    Run(run::FeedBack),
}

/// Render command line
pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    use ratatui::style::Stylize;

    let cfg = crate::get_config();
    if let crate::state::CurrentInput::Cmd(cmd_state) = &app_state.current_input {
        // Select background color based on the command type
        let (color, content) = match cmd_state {
            crate::state::CmdState::Input(content) => {
                (cfg.theme.fg.important, " :".to_owned() + content)
            }
            crate::state::CmdState::Success => (
                cfg.theme.fg.less,
                " Command executed successfully".to_owned(),
            ),
            crate::state::CmdState::Error(content) => (cfg.theme.fg.err, " ".to_owned() + content),
            crate::state::CmdState::Doc(content) => (cfg.theme.fg.common, " ".to_owned() + content),
        };
        f.render_widget(ratatui::widgets::Paragraph::new(content).fg(color), area);
    }
}

/// Handle command line key events
pub(crate) fn handle_key(
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

// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Command line

mod run;

/// Render command line
pub(crate) fn render(
    f: &mut ratatui::Frame,
    app_state: &mut crate::state::AppState,
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
    app_state: &mut crate::state::AppState,
) {
    use crossterm::event::{KeyCode, KeyEventKind};

    if let crate::state::CurrentInput::Cmd(cmd_state) = &mut app_state.current_input {
        // Only handle key when you press expect release
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                // Add char
                KeyCode::Char(key) => {
                    if let crate::state::CmdState::Input(content) = cmd_state {
                        content.push(key);
                    }
                }
                KeyCode::Backspace => {
                    if let crate::state::CmdState::Input(content) = cmd_state {
                        if content.is_empty() {
                            app_state.current_input = crate::state::CurrentInput::None;
                        } else {
                            content.pop();
                        }
                    }
                }
                // Run command
                KeyCode::Enter => {
                    run::try_run_cmd(app_state);
                }
                // Exit command input state
                KeyCode::Esc => {
                    app_state.current_input = crate::state::CurrentInput::None;
                }
                _ => {} // Unknown key
            }
        }
    }
}

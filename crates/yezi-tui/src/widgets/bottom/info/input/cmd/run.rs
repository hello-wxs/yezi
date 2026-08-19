// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

/// The yezi-tui command line interface
#[derive(Clone, Debug, bpaf::Bpaf)]
#[bpaf(options, version)]
enum Commands {
    /// Quit the app
    #[bpaf(command("quit"))]
    Quit,
    // Quit the app
    #[bpaf(command("q"))]
    Q,
    /// Open a library
    #[bpaf(command("open"))]
    Open {
        /// Lib path
        #[bpaf(positional)]
        path: String,
    },
    /// Open a library
    #[bpaf(command("o"))]
    O {
        /// Lib path
        #[bpaf(positional)]
        path: String,
    },
    /// Change view
    #[bpaf(command("view"))]
    View {
        /// Page to view
        #[bpaf(positional)]
        page: String,
    },
}

/// Run command
pub(crate) fn try_run_cmd(app_state: &mut crate::state::AppState) {
    if let crate::state::CurrentInput::Cmd(ref cmd_state) = app_state.current_input {
        match shell_words::split(cmd_state.get_input().unwrap()) {
            Ok(args) => {
                app_state.current_input = crate::state::CurrentInput::Cmd(run_cmd(app_state, args));
            }
            Err(e) => {
                app_state.current_input = crate::state::CurrentInput::Cmd(
                    crate::state::CmdState::Error(format!("failed to parse: {e}")),
                );
            }
        }
    }
}
fn run_cmd(app_state: &mut crate::state::AppState, args: Vec<String>) -> crate::state::CmdState {
    match commands().run_inner(&*args) {
        Ok(cli) => match cli {
            Commands::Quit | Commands::Q => {
                app_state.is_running = false;
                crate::state::CmdState::Success
            }
            Commands::Open { path } | Commands::O { path } => {
                let path = std::path::PathBuf::from(path);
                match yezi_data::note::Lib::read(path.clone()) {
                    Ok(lib) => {
                        app_state.data.push(lib);
                        app_state.current_view.set_libs();
                        crate::state::CmdState::Success
                    }
                    Err(e) => {
                        log::warn!("failed to open the lib: {e}");
                        crate::state::CmdState::Error(format!("{e}"))
                    }
                }
            }
            Commands::View { page } => match page.as_str() {
                "home" => {
                    app_state.current_view.set_home();
                    crate::state::CmdState::Success
                }
                "libs" => {
                    app_state.current_view.set_libs();
                    crate::state::CmdState::Success
                }
                _ => crate::state::CmdState::Error(format!("unknown page: {page}")),
            },
        },
        // Failed to parse the command
        Err(err) => {
            // Output the error message
            use bpaf::ParseFailure;
            match err {
                // Real error
                ParseFailure::Stderr(doc) => crate::state::CmdState::Error(format!("{}: ", doc)),
                // Document or shell
                ParseFailure::Stdout(doc, show_hint) => {
                    if show_hint {
                        crate::state::CmdState::Error("Don't use this".into())
                    } else {
                        crate::state::CmdState::Doc(doc.to_string())
                    }
                }
                // Shell completion
                ParseFailure::Completion(_) => {
                    crate::state::CmdState::Error("Don't use this".into())
                }
            }
        }
    }
}

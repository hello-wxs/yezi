// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) struct FeedBack {
    cmd_state: crate::state::CmdState,
    operation: Operation,
}

pub(crate) enum Operation {
    None,
    Quit,
    Open(yezi_data::note::Node),
    GoTo(crate::state::AppView),
}

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
pub(crate) fn try_run_cmd(app_state: &crate::state::AppState) -> FeedBack {
    if let crate::state::CurrentInput::Cmd(ref cmd_state) = app_state.current_input {
        match shell_words::split(cmd_state.get_input().unwrap()) {
            Ok(args) => run_cmd(args),
            Err(e) => FeedBack {
                cmd_state: crate::state::CmdState::Error(format!("failed to parse: {e}")),
                operation: Operation::None,
            },
        }
    } else {
        unreachable!()
    }
}

fn run_cmd(args: Vec<String>) -> FeedBack {
    match commands().run_inner(&*args) {
        Ok(cli) => match cli {
            Commands::Quit | Commands::Q => FeedBack {
                cmd_state: crate::state::CmdState::Success,
                operation: Operation::Quit,
            },
            Commands::Open { path } | Commands::O { path } => {
                let path = std::path::PathBuf::from(path);
                match yezi_data::note::Node::read(&path) {
                    Ok(note) => FeedBack {
                        cmd_state: crate::state::CmdState::Success,
                        operation: Operation::Open(note),
                    },
                    Err(e) => {
                        log::warn!("failed to open the lib: {e}");
                        FeedBack {
                            cmd_state: crate::state::CmdState::Error(format!("{e}")),
                            operation: Operation::None,
                        }
                    }
                }
            }
            Commands::View { page } => match page.as_str() {
                "home" => FeedBack {
                    cmd_state: crate::state::CmdState::Success,
                    operation: Operation::GoTo(crate::state::AppView::Home),
                },
                "select" => FeedBack {
                    cmd_state: crate::state::CmdState::Success,
                    operation: Operation::GoTo(crate::state::AppView::Select),
                },
                _ => FeedBack {
                    cmd_state: crate::state::CmdState::Error(format!("unknown page: {page}")),
                    operation: Operation::None,
                },
            },
        },
        // Failed to parse the command
        Err(err) => {
            // Output the error message
            use bpaf::ParseFailure;
            match err {
                // Real error
                ParseFailure::Stderr(doc) => FeedBack {
                    cmd_state: crate::state::CmdState::Error(format!("{}: ", doc)),
                    operation: Operation::None,
                },
                // Document or shell
                ParseFailure::Stdout(doc, show_hint) => {
                    if show_hint {
                        FeedBack {
                            cmd_state: crate::state::CmdState::Error("Don't use this".into()),
                            operation: Operation::None,
                        }
                    } else {
                        FeedBack {
                            cmd_state: crate::state::CmdState::Doc(doc.to_string()),
                            operation: Operation::None,
                        }
                    }
                }
                // Shell completion
                ParseFailure::Completion(_) => FeedBack {
                    cmd_state: crate::state::CmdState::Error("Don't use this".into()),
                    operation: Operation::None,
                },
            }
        }
    }
}

pub fn update(app_state: &mut crate::state::AppState, feedback: FeedBack) {
    app_state.current_input = crate::state::CurrentInput::Cmd(feedback.cmd_state);
    match feedback.operation {
        Operation::None => {}
        Operation::Quit => app_state.is_running = false,
        Operation::Open(node) => {
            let yezi_data::note::Node::Fork(ref mut fork) = app_state.data else {
                unreachable!()
            };
            fork.children.push(node);
        }
        Operation::GoTo(view) => {
            app_state.current_view = view;
        }
    }
}

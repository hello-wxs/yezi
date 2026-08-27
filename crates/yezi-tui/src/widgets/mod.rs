// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Pages module

pub(crate) mod bottom;
pub(crate) mod pages;

use anyhow::{Context, Ok};

#[derive(Debug)]
pub(crate) struct Path {
    #[allow(dead_code)]
    pub(crate) bin_path: std::path::PathBuf,
    pub(crate) log_path: std::path::PathBuf,
    pub(crate) config_path: std::path::PathBuf,
}

impl Path {
    pub(crate) fn auto() -> anyhow::Result<Self> {
        if cfg!(feature = "dev") {
            Ok(Self {
                bin_path: std::env::current_exe()?
                    .parent()
                    .context("Path not found")?
                    .into(),
                log_path: "./runtime/log".into(),
                config_path: "./runtime/config".into(),
            })
        } else if cfg!(feature = "portable") {
            let exe_path = std::env::current_exe().context("Failed to get exe path")?;
            let root_path = exe_path
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .context("Path not found")?;

            Ok(Self {
                bin_path: root_path.join("bin"),
                log_path: root_path.join("log"),
                config_path: root_path.join("config"),
            })
        } else {
            use etcetera::AppStrategy;
            let args = etcetera::app_strategy::AppStrategyArgs {
                top_level_domain: "local".to_string(),
                author: "hello_wxs".to_string(),
                app_name: "yezi".to_string(),
            };
            let strategy = etcetera::app_strategy::Xdg::new(args)?;
            Ok(Self {
                bin_path: std::env::current_exe()
                    .context("Failed to get exe path")?
                    .parent()
                    .context("Path not found")?
                    .into(),
                log_path: strategy.state_dir().unwrap(),
                config_path: strategy.config_dir(),
            })
        }
    }
}

pub(super) enum FeedBack {
    OpenCmd,
    Bottom(bottom::FeedBack),
    Pages(pages::FeedBack),
}

pub(super) fn render(
    f: &mut ratatui::Frame,
    app_state: &crate::state::AppState,
    area: ratatui::layout::Rect,
) {
    use ratatui::prelude::Stylize;

    let cfg = crate::get_config();
    let background = ratatui::widgets::Block::default().bg(cfg.theme.bg);
    f.render_widget(background, area);

    // Split Screen
    let [page_area, bottom_area] = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Min(0),
        ratatui::layout::Constraint::Length(4),
    ])
    .areas(area);
    // Render pages
    pages::render(f, app_state, page_area);
    bottom::render(f, app_state, bottom_area);
}

pub(super) fn handle_key(
    key_event: crossterm::event::KeyEvent,
    app_state: &crate::state::AppState,
) -> FeedBack {
    if key_event.kind == crossterm::event::KeyEventKind::Press
        && key_event.code == crossterm::event::KeyCode::Char(':')
    {
        return FeedBack::OpenCmd;
    }
    if crate::widgets::bottom::info::input::Input::None == app_state.current_input {
        FeedBack::Pages(pages::handle_key(key_event, app_state))
    } else {
        FeedBack::Bottom(bottom::handle_key(key_event, app_state))
    }
}

pub(super) fn update(app_state: &mut crate::state::AppState, feedback: FeedBack) {
    match feedback {
        FeedBack::OpenCmd => {
            app_state.current_input = crate::widgets::bottom::info::input::Input::Cmd(
                crate::widgets::bottom::info::input::cmd::State::Input(String::new()),
            );
        }
        FeedBack::Bottom(bottom_feedback) => {
            bottom::update(app_state, bottom_feedback);
        }
        FeedBack::Pages(pages_feedback) => {
            pages::update(app_state, pages_feedback);
        }
    }
}

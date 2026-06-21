// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! TUI page entrance

mod app;
mod cfg;
mod feat_check;
mod logger;
mod state;
mod terminal;
mod widgets;
pub(crate) use cfg::get_config;

fn main() -> anyhow::Result<()> {
    let mut app_state = state::AppState::new()?;
    logger::init_log(&app_state)?;
    cfg::load_config(&app_state.path.config_path.join("yezi-tui.ron"));
    let mut terminal = terminal::init_terminal().inspect_err(|e| log::error!("{e}"))?;
    app::start(&mut app_state, &mut terminal).inspect_err(|e| log::error!("{e}"))?;
    terminal::clear_terminal(&mut terminal).inspect_err(|e| log::error!("{e}"))?;
    Ok(())
}

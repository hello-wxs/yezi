// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#![doc = include_str!("../README.md")]

mod app;
mod cfg;
mod feat_check;
mod logger;
mod state;
mod widgets;
pub(crate) use cfg::get_config;

pub fn run() -> anyhow::Result<()> {
    let mut app_state = state::AppState::new()?;
    logger::init_log(&app_state)?;
    cfg::load_config(&app_state.path.config_path.join("yezi-tui.ron"));
    let mut terminal = init_terminal().inspect_err(|e| log::error!("{e}"))?;
    app::start(&mut app_state, &mut terminal).inspect_err(|e| log::error!("{e}"))?;
    clear_terminal(&mut terminal).inspect_err(|e| log::error!("{e}"))?;
    Ok(())
}

pub(crate) fn init_terminal()
-> std::io::Result<ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::cursor::Hide)?;
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    crossterm::execute!(stdout, crossterm::event::EnableMouseCapture)?;
    let terminal = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(stdout))?;
    Ok(terminal)
}

pub(crate) fn clear_terminal(
    terminal: &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
) -> std::io::Result<()> {
    let stdout = &mut *terminal.backend_mut();
    crossterm::execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::execute!(stdout, crossterm::event::DisableMouseCapture)?;
    crossterm::terminal::disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}

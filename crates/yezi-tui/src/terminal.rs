// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

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

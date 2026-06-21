// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use smol::stream::StreamExt;

pub(crate) fn start(
    app_state: &mut crate::state::AppState,
    terminal: &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
) -> anyhow::Result<()> {
    smol::block_on(async {
        let mut event_stream = crossterm::event::EventStream::new();
        let mut dirty = true;
        while app_state.is_running {
            // Draw screen
            if dirty {
                terminal.draw(|f| {
                    crate::widgets::render(f, app_state, f.area());
                })?;
            }
            dirty = true; // By default, there are no dirty areas.
            // Check key state
            if let Some(event_result) = event_stream.next().await {
                match event_result? {
                    crossterm::event::Event::Key(key_event) => {
                        crate::widgets::handle_key(key_event, app_state);
                    }
                    crossterm::event::Event::Resize(_, _) => {
                        dirty = true;
                        log::trace!("resize")
                    }
                    _ => {
                        dirty = false;
                        log::trace!("unknown operation");
                    }
                }
            }
            app_state.buddy.try_tire()?;
        }
        Ok(())
    })
}

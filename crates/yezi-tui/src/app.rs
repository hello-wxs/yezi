// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Ok;
use smol::stream::StreamExt;

pub(crate) fn start(
    app_state: &mut crate::state::AppState,
    terminal: &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
) -> anyhow::Result<()> {
    smol::block_on(async {
        let mut event_stream = crossterm::event::EventStream::new();
        let mut dirty = true;
        while app_state.is_running {
            if dirty {
                terminal.draw(|f| {
                    crate::widgets::render(f, &*app_state, f.area());
                })?;
            }
            if let Some(event_result) = event_stream.next().await {
                match event_result? {
                    crossterm::event::Event::Key(key_event) => {
                        dirty = true;
                        let feedback = crate::widgets::handle_key(key_event, app_state);
                        crate::widgets::update(app_state, feedback);
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

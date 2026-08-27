// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Ok;
use smol::stream::StreamExt;

#[derive(Debug)]
pub(crate) struct State {
    /// Continue running
    pub(crate) is_running: bool,
    /// Current view
    pub(crate) current_view: crate::widgets::pages::State,
    /// Command state
    pub(crate) current_input: crate::widgets::bottom::info::input::Input,
    /// Libraries list
    pub(crate) data: yezi_data::note::Node,
    /// Selected state
    pub(crate) _selected: Option<yezi_data::note::Node>,
    /// App path
    pub(crate) path: crate::widgets::Path,
    /// Buddy state
    pub(crate) buddy: yezi_buddy::Buddy,
}

impl State {
    pub(crate) fn new() -> anyhow::Result<Self> {
        Ok(Self {
            is_running: true,
            current_view: crate::widgets::pages::State::default(),
            current_input: crate::widgets::bottom::info::input::Input::None,
            data: yezi_data::note::Node::default(),
            _selected: None,
            path: crate::widgets::Path::auto()?,
            buddy: yezi_buddy::Buddy::new("hello_wxs".to_string(), None, 8),
        })
    }
}

pub(crate) fn start(
    app_state: &mut State,
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

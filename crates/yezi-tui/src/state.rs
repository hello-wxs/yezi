// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

/// App state
#[derive(Debug)]
pub(crate) struct AppState {
    /// Continue running
    pub(crate) is_running: bool,
    /// Current view
    pub(crate) current_view: crate::widgets::pages::View,
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

impl AppState {
    pub(crate) fn new() -> anyhow::Result<Self> {
        Ok(Self {
            is_running: true,
            current_view: crate::widgets::pages::View::Home,
            current_input: crate::widgets::bottom::info::input::Input::None,
            data: yezi_data::note::Node::default(),
            _selected: None,
            path: crate::widgets::Path::auto()?,
            buddy: yezi_buddy::Buddy::new("hello_wxs".to_string(), None, 8),
        })
    }
}

// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

mod input;
mod learn;
mod path;
mod view;

pub(crate) use input::*;
pub(crate) use learn::*;
pub(crate) use view::*;

/// App state
#[derive(Debug)]
pub(crate) struct AppState {
    /// Continue running
    pub(crate) is_running: bool,
    /// Current view
    pub(crate) current_view: AppView,
    /// Command state
    pub(crate) current_input: CurrentInput,
    /// Libraries list
    pub(crate) data: yezi_data::note::Node,
    /// Selected state
    pub(crate) _selected: Option<yezi_data::note::Node>,
    /// App path
    pub(crate) path: path::Path,
    /// Buddy state
    pub(crate) buddy: yezi_buddy::Buddy,
}

impl AppState {
    pub(crate) fn new() -> anyhow::Result<Self> {
        Ok(Self {
            is_running: true,
            current_view: AppView::Home,
            current_input: CurrentInput::None,
            data: yezi_data::note::Node::default(),
            _selected: None,
            path: path::Path::auto()?,
            buddy: yezi_buddy::Buddy::new("hello_wxs".to_string(), None, 8),
        })
    }
    pub(crate) fn _learn_state(&self) -> Option<&LearnState> {
        match self.current_view {
            AppView::Learn(ref state) => Some(state),
            _ => None,
        }
    }
    pub(crate) fn _learn_state_mut(&mut self) -> Option<&mut LearnState> {
        match self.current_view {
            AppView::Learn(ref mut state) => Some(state),
            _ => None,
        }
    }
}

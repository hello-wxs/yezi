// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use yezi_data::note::{Book, Entry, Lib};
mod input;
mod learn;
mod path;
mod selected;
mod view;

pub(crate) use input::*;
pub(crate) use learn::*;
pub(crate) use selected::*;
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
    pub(crate) data: Vec<Lib>,
    /// Selected state
    pub(crate) selected: Sellected,
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
            data: Vec::new(),
            selected: Sellected::default(),
            path: path::Path::auto()?,
            buddy: yezi_buddy::Buddy::new("hello_wxs".to_string(), None, 8),
        })
    }
}

impl AppState {
    pub(crate) fn current_lib(&self) -> Option<&Lib> {
        self.data.get(self.selected.lib.selected()?)
    }
    pub(crate) fn current_book(&self) -> Option<&Book> {
        self.current_lib()?.get_book(self.selected.book.selected()?)
    }
    pub(crate) fn current_entry(&self) -> Option<&Entry> {
        self.current_book()?
            .get_entry(self.selected.entry.selected()?)
    }
}

impl AppState {
    pub(crate) fn learn_state(&self) -> Option<&LearnState> {
        match self.current_view {
            AppView::Learn(ref state) => Some(state),
            _ => None,
        }
    }
    pub(crate) fn learn_state_mut(&mut self) -> Option<&mut LearnState> {
        match self.current_view {
            AppView::Learn(ref mut state) => Some(state),
            _ => None,
        }
    }
}

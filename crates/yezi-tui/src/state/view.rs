// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

/// App ciew
#[derive(Debug, PartialEq)]
pub(crate) enum AppView {
    /// Home page
    Home,
    /// Select page
    Select(super::selected::Select),
    /// Learn page
    Learn(super::learn::LearnState),
}

#[allow(dead_code)]
impl AppView {
    pub(crate) fn is_home(&self) -> bool {
        matches!(self, AppView::Home)
    }
    pub(crate) fn is_libs(&self) -> bool {
        matches!(self, AppView::Select(super::selected::Select::Libs))
    }
    pub(crate) fn is_books(&self) -> bool {
        matches!(self, AppView::Select(super::selected::Select::Books))
    }
    pub(crate) fn is_entries(&self) -> bool {
        matches!(self, AppView::Select(super::selected::Select::Entries))
    }
    pub(crate) fn is_learn(&self) -> bool {
        matches!(self, AppView::Learn(_))
    }
}

#[allow(dead_code)]
impl AppView {
    pub(crate) fn set_home(&mut self) {
        *self = AppView::Home;
    }
    pub(crate) fn set_libs(&mut self) {
        *self = AppView::Select(super::selected::Select::Libs);
    }
    pub(crate) fn set_books(&mut self) {
        *self = AppView::Select(super::selected::Select::Books);
    }
    pub(crate) fn set_entries(&mut self) {
        *self = AppView::Select(super::selected::Select::Entries);
    }
    pub(crate) fn set_learn(&mut self) {
        *self = AppView::Learn(super::learn::LearnState::default());
    }
}
impl AppView {
    pub(crate) fn learn_state(&self) -> Option<&super::learn::LearnState> {
        match self {
            AppView::Learn(state) => Some(state),
            _ => None,
        }
    }
}

// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

/// App ciew
#[derive(Debug)]
pub(crate) enum AppView {
    /// Home page
    Home,
    /// Select page
    Select,
    /// Learn page
    Learn(super::learn::LearnState),
}

#[allow(dead_code)]
impl AppView {
    pub(crate) fn is_selecting(&self) -> bool {
        matches!(self, AppView::Select)
    }
    pub(crate) fn is_learn(&self) -> bool {
        matches!(self, AppView::Learn(_))
    }
    pub(crate) fn set_home(&mut self) {
        *self = AppView::Home;
    }
    pub(crate) fn set_select(&mut self) {
        *self = AppView::Select;
    }
    pub(crate) fn set_learn(&mut self) {
        *self = AppView::Learn(super::learn::LearnState::default());
    }
    pub(crate) fn learn_state(&self) -> Option<&super::learn::LearnState> {
        match self {
            AppView::Learn(state) => Some(state),
            _ => None,
        }
    }
}

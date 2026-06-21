// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

/// Learn state
#[derive(Debug, Default, PartialEq)]
pub(crate) struct LearnState {
    /// Currently showing entry
    pub(crate) now_show: usize,
    /// Whether to show tip
    pub(crate) show_tip: bool,
}

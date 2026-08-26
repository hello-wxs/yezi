// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

/// Search state
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SearchState {
    /// Search input is inputting
    #[allow(dead_code)]
    Input(String),
}

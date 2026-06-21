// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

/// Selected state
#[derive(Debug, Default)]
pub(crate) struct Sellected {
    /// Selected library
    pub(crate) lib: ratatui::widgets::ListState,
    /// Selected book
    pub(crate) book: ratatui::widgets::ListState,
    /// Selected entry
    pub(crate) entry: ratatui::widgets::ListState,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Select {
    /// Configuration page
    Libs,
    /// Books page
    Books,
    /// Entries page
    Entries,
}

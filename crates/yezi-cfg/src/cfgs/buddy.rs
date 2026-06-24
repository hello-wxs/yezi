// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

/// Buddy configuration.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Buddy {
    /// Buddy name.
    pub name: Option<String>,
    /// Buddy introduction.
    pub intro: Option<String>,
    /// Whether to show the buddy.
    pub show: bool,
}

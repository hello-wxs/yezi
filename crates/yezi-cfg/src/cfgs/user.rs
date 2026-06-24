// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

/// User configuration.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct User {
    /// User name.
    pub name: Option<String>,
    /// User introduction.
    pub intro: Option<String>,
}

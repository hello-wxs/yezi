// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct User {
    pub name: Option<String>,
    pub intro: Option<String>,
}

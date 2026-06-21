// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! The yezi-cfg crate provides configuration management for the yezi project.

pub mod cfgs;
pub mod error;

use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Cfg {
    pub user: cfgs::user::User,
    pub buddy: cfgs::buddy::Buddy,
    pub theme: cfgs::theme::Theme,
}

impl Cfg {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, error::Error> {
        let content = std::fs::read_to_string(path)?;
        let res = ron::from_str(&content)?;
        Ok(res)
    }
    pub fn write_default<P: AsRef<std::path::Path>>(path: P) -> Result<(), error::Error> {
        std::fs::write(path, include_str!("../assest/yezi-tui.ron"))?;
        Ok(())
    }
}

// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

/// Buddy configuration.
mod buddy;
/// Theme configuration.
mod theme;
/// User configuration.
mod user;

use serde::Deserialize;

/// Configuration struct.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Cfg {
    /// User configuration.
    pub user: crate::cfgs::user::User,
    /// Buddy configuration.
    pub buddy: crate::cfgs::buddy::Buddy,
    /// Theme configuration.
    pub theme: crate::cfgs::theme::Theme,
}

impl Cfg {
    /// Load configuration from a file.
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, crate::error::Error> {
        let content = std::fs::read_to_string(path)?;
        let res = ron::from_str(&content)?;
        Ok(res)
    }
    /// Write default configuration to a file.
    pub fn write_default<P: AsRef<std::path::Path>>(path: P) -> Result<(), crate::error::Error> {
        std::fs::write(path, include_str!("../../assest/yezi-tui.ron"))?;
        Ok(())
    }
}

// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;

/// Theme configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct Theme {
    /// Background color.
    pub bg: ratatui_core::style::Color,
    /// Foreground color.
    pub fg: Font,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: ratatui_core::style::Color::Rgb(47, 47, 47),
            fg: Font::default(),
        }
    }
}

/// Font configuration.
#[derive(Debug, Deserialize, Clone)]
pub struct Font {
    /// Error color.
    pub err: ratatui_core::style::Color,
    /// Warning color.
    pub warn: ratatui_core::style::Color,
    /// Important color.
    pub important: ratatui_core::style::Color,
    /// Common color.
    pub common: ratatui_core::style::Color,
    /// Less color.
    pub less: ratatui_core::style::Color,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            err: ratatui_core::style::Color::Rgb(255, 0, 0),
            warn: ratatui_core::style::Color::Rgb(255, 255, 0),
            important: ratatui_core::style::Color::Rgb(255, 255, 255),
            common: ratatui_core::style::Color::Rgb(191, 191, 191),
            less: ratatui_core::style::Color::Rgb(127, 127, 127),
        }
    }
}

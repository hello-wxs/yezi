// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

/// Command state
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CmdState {
    /// Command line is inputting
    Input(String),
    /// Command line execution succeeded
    Success,
    /// Command line execution error
    Error(String),
    /// With doc
    Doc(String),
}

impl Default for CmdState {
    fn default() -> Self {
        Self::Input(String::new())
    }
}

impl CmdState {
    pub(crate) fn get_input(&self) -> Option<&str> {
        match self {
            Self::Input(input) => Some(input),
            _ => None,
        }
    }
}

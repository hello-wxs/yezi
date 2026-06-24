// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

/// Error types for the configuration module.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// File open error.
    #[error("File open error: {0}")]
    FileOpenError(#[from] std::io::Error),
    /// Serde error.
    #[error("Serde error: {0}")]
    SerdeError(#[from] ron::error::SpannedError),
}

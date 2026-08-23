// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Provides the Error type
/// The yezi-data error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// File read error
    #[error("file read error")]
    IOError(#[from] std::io::Error),
    /// Unkown file extension
    #[error("unknown file extension")]
    UnknownExtension,
    /// YAML error
    #[error("yaml error")]
    YamlParseError(#[from] yaml_serde::Error),
}
pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

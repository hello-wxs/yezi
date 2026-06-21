// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Provides the Error type
/// The yezi-data error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// File read error
    #[error("file read error")]
    ReadError(std::io::Error),
    /// File write error
    #[error("file write error")]
    WriteError(std::io::Error),
    /// Unkown file extension
    #[error("unknown file extension")]
    UnknownExtension,
    /// InvalidUtf8
    #[error("invalid utf8")]
    InvalidUtf8(std::path::PathBuf),
    /// TOML deserialize error
    #[error("toml parse error")]
    TomlParseError(toml::de::Error),
    /// JSON deserialize error
    #[error("json parse error")]
    JsonParseError(serde_json::Error),
    /// YAML deserialize error
    #[error("yaml parse error")]
    YamlParseError(yaml_serde::Error),
    /// TOML serialize error
    #[error("toml serialize error")]
    SerializeError(toml::ser::Error),
    /// RON deserialize error
    #[error("ron parse error")]
    RonParseError(ron::error::Error),
}

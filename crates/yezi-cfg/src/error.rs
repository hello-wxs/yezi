// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("File open error: {0}")]
    FileOpenError(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] ron::error::SpannedError),
}

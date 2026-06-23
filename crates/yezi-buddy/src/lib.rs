// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#![doc = include_str!("../README.md")]

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Assest module
mod assest;
/// Buddy module
mod buddy;

pub use buddy::Buddy;

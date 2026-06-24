// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#![doc = include_str!("../README.md")]

pub mod state;
pub use state::*;

pub mod libs;
pub use libs::*;

pub mod books;
pub use books::*;

pub mod entries;
pub use entries::*;

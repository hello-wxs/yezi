// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod state;
pub use state::*;

mod libs;
pub use libs::*;

mod books;
pub use books::*;

mod entries;
pub use entries::*;

// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! The `yezi-buddy` crate provides a buddy that can generate random states.
//! You can create a buddy with a custom name and buddy_name.
//!
//! # Examples
//!
//! You can both provide a user name and a buddy name.
//!
//! ```
//! use yezi_buddy::Buddy;
//!
//! let mut buddy = Buddy::new("hello_wxs".to_string(), Some("Biu".to_string()), 8);
//! ```
//!
//! If you don't provide a buddy name, the type name will be used as the buddy name.
//! ```
//! use yezi_buddy::Buddy;
//!
//! let mut buddy = Buddy::new("hello_wxs".to_string(), None, 8);
//!
//! ```

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Assest module
mod assest;
/// Buddy module
mod buddy;

pub use buddy::Buddy;

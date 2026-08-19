// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Provides the Entry type

use serde::{Deserialize, Serialize};

/// Represents an entry with a key, value, and tip.
///
/// # Examples
///
/// You can create an entry using the [`Entry::new`] method with string literals.
///
/// ```
/// use yezi_data::Entry;
///
/// let entry = Entry::new("key".into(), "value".into(), "tip".into());
/// ```
///
/// You can use many functions to get or set the entry's fields.
/// Here are some examples, but it only shows a few of the many functions available.
///
/// ```
/// use yezi_data::Entry;
///
/// let entry = Entry::new("key".into(), "value".into(), "tip".into());
///
/// let key = entry.get_key();
/// assert_eq!("key", key);
///
/// let value = entry.get_value();
/// assert_eq!("value", value);
///
/// let tip = entry.get_tip();
/// assert_eq!("tip", tip);
/// ```
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Entry {
    /// The unique identifier of the entry.
    #[serde(default = "ulid::Ulid::generate")]
    id: ulid::Ulid,
    /// The key of the entry.
    key: String,
    /// The value of the entry.
    value: String,
    /// The tip of the entry.
    tip: String,
}

impl Entry {
    /// Create a new entry with the given key, value, and tip.
    pub fn new(key: String, value: String, tip: String) -> Self {
        Self {
            id: ulid::Ulid::generate(),
            key,
            value,
            tip,
        }
    }
    /// Get a reference to the entry's key.
    pub fn get_key(&self) -> &String {
        &self.key
    }
    /// Get a mutable reference to the entry's key.
    pub fn get_mut_key(&mut self) -> &mut String {
        &mut self.key
    }
    /// Set the entry's key.
    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }
    /// Get a reference to the entry's value.
    pub fn get_value(&self) -> &String {
        &self.value
    }
    /// Get a mutable reference to the entry's value.
    pub fn get_mut_value(&mut self) -> &mut String {
        &mut self.value
    }
    /// Set the entry's value.
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    /// Get a reference to the entry's tip.
    pub fn get_tip(&self) -> &String {
        &self.tip
    }
    /// Get a mutable reference to the entry's tip.
    pub fn get_mut_tip(&mut self) -> &mut String {
        &mut self.tip
    }
    /// Set the entry's tip.
    pub fn set_tip(&mut self, tip: String) {
        self.tip = tip;
    }
}

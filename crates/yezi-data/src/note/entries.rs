// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Provides the Entry type

use serde::{Deserialize, Serialize};

/// Represents an entry with dynamic content.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Entry(yaml_serde::Value);

impl Entry {
    /// Retrieves a value by key, returning `None` if the key is not found or the value is not a string.
    pub fn get(&self, key: &str) -> Option<&str> {
        let mut value = &self.0;
        for part in key.split(".") {
            value = value.get(part)?;
        }
        value.as_str()
    }
}

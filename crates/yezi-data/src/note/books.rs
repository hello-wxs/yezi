// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Provides the Book type

use serde::{Deserialize, Serialize};

use super::Entry;

/// Represents a book with a name, description, and entries.
///
/// # Examples
///
/// You can create a book using the [`Book::new`] method with string literals.
///
/// ```
/// use yezi_data::Book;
///
/// let mut book = Book::new("My Book".to_string(), "A book in yezi".to_string());
/// ```
///
/// There are many functions available to get or set the book's fields.
/// Here are some examples, but it only shows a few of the many functions available.
///
/// ```
/// use yezi_data::Book;
///
/// let book = Book::new("My Book".to_string(), "A book in yezi".to_string());
/// let name = book.get_name();
/// assert_eq!("My Book", name);
///
/// let description = book.get_description();
/// assert_eq!("A book in yezi", description);
/// ```
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Book {
    /// The unique identifier of the entry.
    #[serde(default = "ulid::Ulid::generate")]
    id: ulid::Ulid,
    /// The name of the book.
    name: String,
    /// The description of the book.
    description: String,
    /// The entries of the book.
    entries: Vec<Entry>,
}

impl Book {
    /// Create a new book with the given name and description.
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: ulid::Ulid::generate(),
            name,
            description,
            entries: Vec::new(),
        }
    }
    /// Get a reference to the book's name.
    pub fn get_name(&self) -> &String {
        &self.name
    }
    /// Get a mutable reference to the book's name.
    pub fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }
    /// Set the book's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    /// Get a reference to the book's description.
    pub fn get_description(&self) -> &String {
        &self.description
    }
    /// Get a mutable reference to the book's description.
    pub fn get_mut_description(&mut self) -> &mut String {
        &mut self.description
    }
    /// Set the book's description.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}
impl Book {
    /// Get a reference to the book's entries.
    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }
    /// Get a mutable reference to the book's entries.
    pub fn get_mut_entries(&mut self) -> &mut Vec<Entry> {
        &mut self.entries
    }
    /// Set the book's entries.
    pub fn set_entries(&mut self, entries: Vec<Entry>) {
        *self.get_mut_entries() = entries;
    }
    /// Get a reference to an entry by index.
    pub fn get_entry<I>(&self, entry_idx: I) -> Option<&Entry>
    where
        I: Into<usize>,
    {
        self.get_entries().get(entry_idx.into())
    }
    /// Get a mutable reference to an entry by index.
    pub fn get_mut_entry<I>(&mut self, entry_idx: I) -> Option<&mut Entry>
    where
        I: Into<usize>,
    {
        self.get_mut_entries().get_mut(entry_idx.into())
    }
    /// Set an entry by index.
    pub fn set_entry<I>(&mut self, entry_idx: I, entry: Entry) -> Option<()>
    where
        I: Into<usize>,
    {
        self.get_mut_entry(entry_idx.into())
            .map(|mut_entry| *mut_entry = entry)
    }
}

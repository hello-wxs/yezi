// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Provides the Lib type

use super::{Book, Entry};
use crate::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Represents a lib with a name, description, and a list of books.
///
/// # Examples
///
/// You can create a lib using the [`Lib::new`] method with string literals.
///
/// ```
/// use yezi_data::Lib;
///
/// let lib = Lib::new("My Lib".into(), "A lib in yezi".into());
/// ```
///
/// There are many functions available to get or set the book's fields.
/// Here are some examples, but it only shows a few of the many functions available.
///
/// ```
/// use yezi_data::Lib;
///
/// let lib = Lib::new("My Lib".into(), "A lib in yezi".into());
///
/// let name = lib.get_name();
/// assert_eq!("My Lib", name);
///
/// let description = lib.get_description();
/// assert_eq!("A lib in yezi", description);
/// ```
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Lib {
    /// The unique identifier of the lib.
    #[serde(default = "ulid::Ulid::generate")]
    id: ulid::Ulid,
    /// The name of the lib.
    name: String,
    /// The description of the lib.
    description: String,
    /// The books of the lib.
    books: Vec<Book>,
}

impl Lib {
    /// Create a new lib with the given name and description.
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: ulid::Ulid::generate(),
            name,
            description,
            books: Vec::new(),
        }
    }
    /// Read a lib from a file.
    pub fn read(file_path: PathBuf) -> Result<Self, Error> {
        let content = fs::read_to_string(&file_path).map_err(Error::ReadError)?;
        let lib: Self = yaml_serde::from_str(&content).map_err(Error::YamlParseError)?;
        lib.write(file_path)?;
        Ok(lib)
    }
}

impl Lib {
    /// Get a reference to the lib's name.
    pub fn get_name(&self) -> &String {
        &self.name
    }
    /// Get a mutable reference to the lib's name.
    pub fn get_mut_name(&mut self) -> &mut String {
        &mut self.name
    }
    /// Set the lib's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    /// Get a reference to the lib's description.
    pub fn get_description(&self) -> &String {
        &self.description
    }
    /// Get a mutable reference to the lib's description.
    pub fn get_mut_description(&mut self) -> &mut String {
        &mut self.description
    }
    /// Set the lib's description.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}
impl Lib {
    /// Get a reference to the lib's books.
    pub fn get_books(&self) -> &Vec<Book> {
        &self.books
    }
    /// Get a mutable reference to the lib's books.
    pub fn get_mut_books(&mut self) -> &mut Vec<Book> {
        &mut self.books
    }
    /// Set the lib's books.
    pub fn set_books(&mut self, books: Vec<Book>) {
        self.books = books;
    }
    /// Get a reference to a lib's book at the given index.
    pub fn get_book<I>(&self, book_idx: I) -> Option<&Book>
    where
        I: Into<usize>,
    {
        self.get_books().get(book_idx.into())
    }
    /// Get a mutable reference to a lib's book at the given index.
    pub fn get_mut_book<I>(&mut self, book_idx: I) -> Option<&mut Book>
    where
        I: Into<usize>,
    {
        self.get_mut_books().get_mut(book_idx.into())
    }
    /// Set a lib's book at the given index.
    pub fn set_book<I>(&mut self, book_idx: I, book: Book) -> Option<()>
    where
        I: Into<usize>,
    {
        self.get_mut_book(book_idx).map(|mut_book| *mut_book = book)
    }
}
impl Lib {
    /// Get the entry at the given book and entry idxes.
    pub fn get_entries<I>(&self, book_idx: I) -> Option<&Vec<Entry>>
    where
        I: Into<usize>,
    {
        self.get_book(book_idx.into())
            .map(|book| book.get_entries())
    }
    /// Get a mutable reference to the entry at the given book and entry idxes.
    pub fn get_mut_entries<I>(&mut self, book_idx: I) -> Option<&mut Vec<Entry>>
    where
        I: Into<usize>,
    {
        self.get_mut_book(book_idx.into())
            .map(|book| book.get_mut_entries())
    }
    /// Set the book's entries at the given indexes.
    pub fn set_entries<I>(&mut self, book_idx: I, entries: Vec<Entry>) -> Option<()>
    where
        I: Into<usize>,
    {
        self.get_mut_book(book_idx.into())
            .map(|mut_book| *mut_book.get_mut_entries() = entries)
    }
    /// Get the entry at the given book and entry idxes.
    pub fn get_entry<I>(&self, book_idx: I, entry_idx: I) -> Option<&Entry>
    where
        I: Into<usize>,
    {
        self.get_book(book_idx)
            .and_then(|book| book.get_entry(entry_idx))
    }
    /// Get a mutable reference to the entry at the given book and entry idxes.
    pub fn get_mut_entry<I>(&mut self, book_idx: I, entry_idx: I) -> Option<&mut Entry>
    where
        I: Into<usize>,
    {
        self.get_mut_book(book_idx)
            .and_then(|book| book.get_mut_entry(entry_idx))
    }
    /// Set the entry at the given book and entry idxes.
    pub fn set_entry<I>(&mut self, book_idx: I, entry_idx: I, entry: Entry) -> Option<()>
    where
        I: Into<usize>,
    {
        self.get_mut_entry(book_idx, entry_idx)
            .map(|mut_entry| *mut_entry = entry)
    }
}
impl Lib {
    /// Write the lib's data to a TOML file.
    pub fn write(&self, file_path: std::path::PathBuf) -> Result<(), Error> {
        let content = yaml_serde::to_string(&self).map_err(Error::YamlParseError)?;
        fs::write(&file_path, content).map_err(Error::WriteError)?;
        Ok(())
    }
}

// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Provides the Lib type

use crate::*;
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
    #[serde(default = "uuid::Uuid::now_v7")]
    id: uuid::Uuid,
    /// The name of the lib.
    name: String,
    /// The description of the lib.
    description: String,
    /// The books of the lib.
    books: Vec<Book>,
    /// Sign the file extension of the lib.
    #[serde(skip)]
    pub file_extension: FileExtension,
}

impl Lib {
    /// Create a new lib with the given name and description.
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            name,
            description,
            books: Vec::new(),
            file_extension: FileExtension::Toml,
        }
    }
    /// Read a lib from a file.
    pub fn read(file_path: PathBuf) -> Result<Self, Error> {
        let content = fs::read_to_string(&file_path).map_err(Error::ReadError)?;
        let lib = match file_path
            .extension()
            .ok_or(Error::UnknownExtension)?
            .to_str()
            .ok_or(Error::InvalidUtf8(file_path.clone()))?
        {
            "toml" => Self::serde_toml(content),
            "json" => Self::serde_json(content),
            "yaml" => Self::serde_yaml(content),
            "ron" => Self::serde_ron(content),
            _ => Err(Error::UnknownExtension),
        }?;
        lib.write(file_path)?;
        Ok(lib)
    }
    /// Serde a lib from a TOML file.
    fn serde_toml(content: String) -> Result<Self, Error> {
        let mut lib: Self = toml::from_str(&content).map_err(Error::TomlParseError)?;
        lib.file_extension = FileExtension::Toml;
        Ok(lib)
    }
    /// Serde a lib from a JSON file.
    fn serde_json(content: String) -> Result<Self, Error> {
        let mut lib: Self = serde_json::from_str(&content).map_err(Error::JsonParseError)?;
        lib.file_extension = FileExtension::Json;
        Ok(lib)
    }
    /// Serde a lib from a YAML file.
    fn serde_yaml(content: String) -> Result<Self, Error> {
        let mut lib: Self = yaml_serde::from_str(&content).map_err(Error::YamlParseError)?;
        lib.file_extension = FileExtension::Yaml;
        Ok(lib)
    }
    /// Serde a lib from a RON file.
    fn serde_ron(content: String) -> Result<Self, Error> {
        let mut lib: Self = ron::from_str(&content).map_err(|e| Error::RonParseError(e.into()))?;
        lib.file_extension = FileExtension::Ron;
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
        let content = match self.file_extension {
            FileExtension::Toml => toml::to_string(&self).map_err(Error::SerializeError)?,
            FileExtension::Json => {
                serde_json::to_string_pretty(&self).map_err(Error::JsonParseError)?
            }
            FileExtension::Yaml => yaml_serde::to_string(&self).map_err(Error::YamlParseError)?,
            FileExtension::Ron => {
                ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default())
                    .map_err(Error::RonParseError)?
            }
        };

        fs::write(&file_path, content).map_err(Error::WriteError)?;
        Ok(())
    }
}

/// The file extension of the lib.
#[derive(Clone, Debug, Default)]
pub enum FileExtension {
    /// The TOML file extension.
    Toml,
    /// The JSON file extension.
    Json,
    /// The YAML file extension (default).
    #[default]
    Yaml,
    /// The RON file extension.    
    Ron,
}

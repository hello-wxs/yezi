// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//! A module for yezi-buddy to offer buddy assests with images and sayings.

/// Images assest type
#[derive(Debug)]
pub(crate) struct Images<'a> {
    /// Main image
    pub main: &'a str,
    /// Dozy image
    pub dozy: &'a str,
    /// Hint image
    pub hint: &'a str,
    /// Taunt image
    pub taunt: &'a str,
}

/// Saying assest type
#[derive(Debug)]
pub(crate) struct Say<'a> {
    /// Main saying
    pub main: &'a str,
    /// Dozy saying
    pub dozy: &'a str,
    /// Hint saying
    pub hint: &'a str,
    /// Taunt saying
    pub taunt: &'a str,
}

/// Buddy assest type
#[derive(Debug)]
pub(crate) struct Buddy<'a> {
    /// Name assest
    pub name: &'a str,
    /// Image content
    pub image: Images<'a>,
    /// Image color
    pub color: ratatui_core::style::Color,
    /// Saying assests of the buddy
    pub say: Say<'a>,
}

/// Helper function for parsing RGB color strings
const fn rgb(s: &str) -> ratatui_core::style::Color {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut r = 0;
    while bytes[i] != b',' {
        r = r * 10 + (bytes[i] - b'0') as u16;
        i += 1;
    }
    i += 1;
    let mut g = 0;
    while bytes[i] != b',' {
        g = g * 10 + (bytes[i] - b'0') as u16;
        i += 1;
    }
    i += 1;
    let mut b = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        b = b * 10 + (bytes[i] - b'0') as u16;
        i += 1;
    }
    ratatui_core::style::Color::Rgb(r as u8, g as u8, b as u8)
}

/// Macro for creating a Buddy from a name
macro_rules! from_file {
    ($name:expr) => {
        crate::assest::Buddy {
            name: include_str!(concat!("../assest/different/", $name, "/name.txt")),
            image: crate::assest::Images {
                main: include_str!(concat!("../assest/different/", $name, "/img/main.txt")),
                dozy: include_str!(concat!("../assest/different/", $name, "/img/dozy.txt")),
                hint: include_str!(concat!("../assest/different/", $name, "/img/hint.txt")),
                taunt: include_str!(concat!("../assest/different/", $name, "/img/taunt.txt")),
            },
            color: rgb(include_str!(concat!(
                "../assest/different/",
                $name,
                "/color.txt"
            ))),
            say: crate::assest::Say {
                main: include_str!(concat!("../assest/different/", $name, "/say.txt")),
                dozy: include_str!(concat!("../assest/global/dozy.txt")),
                hint: include_str!(concat!("../assest/global/hint.txt")),
                taunt: include_str!(concat!("../assest/global/taunt.txt")),
            },
        }
    };
}

/// Static array of all available buddies
pub(crate) static BUDDIES: [crate::assest::Buddy; 8] = [
    from_file!("cactus"),
    from_file!("cat"),
    from_file!("dog"),
    from_file!("fox"),
    from_file!("pig"),
    from_file!("tree"),
    from_file!("stump"),
    from_file!("spider"),
];

// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

static _TEMPLATES: std::sync::LazyLock<handlebars::Handlebars> =
    std::sync::LazyLock::new(handlebars::Handlebars::new);

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Template {
    que: String,
    ans: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Templates {
    templates: Vec<Template>,
}

impl Templates {
    pub fn read(path: &str) -> crate::state::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Ok(yaml_serde::from_str(&content)?)
    }
}

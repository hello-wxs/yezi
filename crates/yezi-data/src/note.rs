// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[serde(untagged)]
pub enum Node {
    Leaf(Leaf),
    Fork(Fork),
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct Leaf {
    #[serde(default = "ulid::Ulid::generate")]
    pub id: ulid::Ulid,
    pub content: yaml_serde::Value,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct Fork {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn write(&self, file_path: &std::path::Path) -> crate::state::Result<()> {
        let content = yaml_serde::to_string(self)?;
        std::fs::write(file_path, content)?;
        Ok(())
    }
    pub fn read(file_path: &std::path::Path) -> crate::state::Result<Self> {
        let content = std::fs::read_to_string(file_path)?;
        let node: Self = yaml_serde::from_str(&content)?;
        Ok(node)
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::Fork(Fork::default())
    }
}

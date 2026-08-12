// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::{Context, Ok};

#[derive(Debug)]
pub(crate) struct Path {
    #[allow(dead_code)]
    pub(crate) bin_path: std::path::PathBuf,
    pub(crate) log_path: std::path::PathBuf,
    pub(crate) config_path: std::path::PathBuf,
}

impl Path {
    pub(crate) fn auto() -> anyhow::Result<Self> {
        if cfg!(feature = "dev") {
            Ok(Self {
                bin_path: std::env::current_exe()?
                    .parent()
                    .context("Path not found")?
                    .into(),
                log_path: "./runtime/log".into(),
                config_path: "./runtime/config".into(),
            })
        } else if cfg!(feature = "portable") {
            let exe_path = std::env::current_exe().context("Failed to get exe path")?;
            let root_path = exe_path
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .context("Path not found")?;

            Ok(Self {
                bin_path: root_path.join("bin"),
                log_path: root_path.join("log"),
                config_path: root_path.join("config"),
            })
        } else {
            use etcetera::AppStrategy;
            let args = etcetera::app_strategy::AppStrategyArgs {
                top_level_domain: "local".to_string(),
                author: "hello_wxs".to_string(),
                app_name: "yezi".to_string(),
            };
            let strategy = etcetera::app_strategy::Xdg::new(args)?;
            Ok(Self {
                bin_path: std::env::current_exe()
                    .context("Failed to get exe path")?
                    .parent()
                    .context("Path not found")?
                    .into(),
                log_path: strategy.state_dir().unwrap(),
                config_path: strategy.config_dir(),
            })
        }
    }
}

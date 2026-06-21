// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

static CONFIG: std::sync::OnceLock<yezi_cfg::Cfg> = std::sync::OnceLock::new();

pub(crate) fn load_config(path: &std::path::Path) {
    let cfg = yezi_cfg::Cfg::from_file(path).unwrap_or_else(|e| {
        log::error!("{e}");
        log::info!("Create dir ans write default configure to {:?}", &path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        yezi_cfg::Cfg::write_default(path).unwrap();
        yezi_cfg::Cfg::default()
    });
    CONFIG.set(cfg).unwrap();
}

pub(crate) fn get_config() -> &'static yezi_cfg::Cfg {
    CONFIG.get().unwrap()
}

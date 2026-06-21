// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub(crate) fn init_log(app_state: &crate::state::AppState) -> std::io::Result<()> {
    std::fs::create_dir_all(&app_state.path.log_path)?;
    let log_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(app_state.path.log_path.join("yezi-tui.log"))?;
    let mut builder = env_logger::Builder::new();
    builder
        .parse_env("YEZI_LOG")
        .format_module_path(true)
        .format_file(true)
        .format_line_number(true)
        .target(env_logger::Target::Pipe(Box::new(log_file)));

    if cfg!(feature = "dev") {
        builder
            .filter_level(log::LevelFilter::Debug)
            .format_timestamp_millis()
            .format_target(true)
            .format_indent(Some(4))
            .init();
    } else {
        builder
            .filter_level(log::LevelFilter::Warn)
            .format_timestamp_secs()
            .format_target(false)
            .format_indent(None)
            .init();
    }
    Ok(())
}

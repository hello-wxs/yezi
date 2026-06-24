// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#![doc = include_str!("../README.md")]

fn main() {
    // Get basic pathes.
    let self_path = std::env::current_exe().unwrap();
    let bin_path = self_path.parent().unwrap();

    // Check if yezi-tui exists.
    if !bin_path.join("yezi-tui").exists() {
        use std::io::Write;

        // Ask the user if they want to install yezi-tui.
        print!("Missing yezi-tui, do you want to install it? [Y/N]:");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        if input.trim() != "Y" && input.trim() != "y" {
            return;
        }

        // Install yezi-tui.
        std::process::Command::new("cargo")
            .arg("install")
            .arg("yezi-tui")
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    std::process::Command::new("./yezi-tui")
        .current_dir(bin_path)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

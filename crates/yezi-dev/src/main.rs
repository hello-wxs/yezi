// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use bpaf::Bpaf;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
enum Opts {
    /// Create env and copy bins
    #[bpaf(command("setup"))]
    Setup,
    /// Jump
    #[bpaf(command("jump"))]
    Jump,
}

fn main() {
    let self_path = std::env::current_exe().unwrap();
    let opts = opts().run();
    match opts {
        Opts::Setup => {
            let assest_path = self_path.parent().unwrap().join("assest");
            let binary_path = self_path.parent().unwrap().join("target").join("debug");
            let out_path = self_path.parent().unwrap().join(".yezi");
            let bin_path = out_path.join("bin");
            let log_path = out_path.join("log");
            let cfg_path = out_path.join("cfg");

            // Create env
            std::fs::create_dir_all(bin_path.clone()).unwrap();
            std::fs::create_dir_all(log_path.clone()).unwrap();
            std::fs::create_dir_all(cfg_path.clone()).unwrap();
            // Copy config files
            std::fs::copy(
                assest_path.join("crates/yezi-cfg/assest/yezi-tui.ron"),
                cfg_path.join("yezi-tui.ron"),
            )
            .unwrap();
            // Copy binaries
            let file_names: Vec<String> = vec!["yezi-dbg".to_string(), "yezi-tui".to_string()];
            #[allow(unused_mut)]
            for mut file_name in file_names {
                #[cfg(target_os = "windows")]
                file_name.push_str(".exe");
                std::fs::copy(binary_path.join(&file_name), bin_path.join(file_name)).unwrap();
            }
        }
        Opts::Jump => {
            std::fs::copy(
                self_path.clone(),
                self_path
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(
                        #[cfg(target_os = "windows")]
                        "dev.exe",
                        #[cfg(not(target_os = "windows"))]
                        "dev",
                    ),
            )
            .unwrap();
        }
    }
}

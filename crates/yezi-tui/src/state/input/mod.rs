// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

mod cmd;
mod search;
pub(crate) use cmd::*;

#[derive(Debug)]
pub(crate) enum CurrentInput {
    None,
    Cmd(cmd::CmdState),
    #[allow(dead_code)]
    Search(search::SearchState),
}

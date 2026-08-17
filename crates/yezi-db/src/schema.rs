// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

diesel::table! {
    cards (id) {
        id -> Binary,
        state -> Integer,
        due -> Integer,
        last_review -> Integer,
        stability -> Float,
        difficulty -> Float,
    }
}

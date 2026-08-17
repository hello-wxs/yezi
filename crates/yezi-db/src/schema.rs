// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

diesel::table! {
    cards (id) {
        id -> Binary,
        state -> Integer,
        due -> BigInt,
        last_review -> BigInt,
        stability -> Float,
        difficulty -> Float,
    }
}

diesel::table! {
    reviews (id) {
        id -> Binary,
        card_id -> Binary,
        kind -> Integer,
        review_at -> BigInt,
        spaced_time -> BigInt,
        rating -> Integer,
        taken_time -> BigInt,
    }
}

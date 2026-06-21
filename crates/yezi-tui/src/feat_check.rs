// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#[cfg(all(feature = "dev", feature = "install"))]
compile_error!("You can't open dev and install features");

#[cfg(all(feature = "dev", feature = "portable"))]
compile_error!("You can't open dev and portable features");

#[cfg(all(feature = "install", feature = "portable"))]
compile_error!("You can't open install and portable features");

#[cfg(not(any(feature = "dev", feature = "install", feature = "portable")))]
compile_error!("You must open one of dev, install, or portable features");

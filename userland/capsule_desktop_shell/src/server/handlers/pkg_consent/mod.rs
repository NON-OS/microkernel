// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The consent gate for installing a `.nonos` package: Approve commits it via
//! the installer, any other click dismisses.

mod click;

pub(crate) use click::answer;

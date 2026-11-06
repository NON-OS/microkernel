// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The offer to install NONOS at the start of a live session: what its
//! answer does, and when it comes up.

mod check;
mod click;

pub use check::check;
pub use click::answer;

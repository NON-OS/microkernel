// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The offer to install NONOS at the start of a live session: where it sits,
//! what a click on it does, and when it comes up.

mod check;
mod click;
mod geometry;

pub use check::check;
pub use click::{answer, click};
pub(crate) use geometry::{install_rect, later_rect, panel_rect};

// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The terminal's foreground process group, as TIOCSPGRP sets it, in the
//! guests' own numbering. 0 until a shell sets one.

use core::sync::atomic::{AtomicU32, Ordering};

static FG: AtomicU32 = AtomicU32::new(0);

pub fn set_fg(pgrp: u32) {
    FG.store(pgrp, Ordering::Relaxed);
}

/// The group TIOCSPGRP last named, or None while no shell has set one.
pub fn fg() -> Option<u32> {
    Some(FG.load(Ordering::Relaxed)).filter(|&g| g != 0)
}

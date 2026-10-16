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

//! The wiped power-off (F-88). AdminShutdown wipes on the way down; it is
//! asked for by a power-button press, a thermal zone at _CRT, a critical
//! battery, or the shell through `power.off`, with no confirmation.
//! Firmware's own critical shutdown would cut power without the wipe.

mod button;
pub mod rule;

pub use button::{policy, press, take_button, Presses};

use crate::sys::log::Line;
use nonos_libc::mk_admin_shutdown;

/// One line, then AdminShutdown. Returns only when the kernel refused it,
/// with the errno; the callers then fall back to what ran before.
pub fn power_off(why: &[u8]) -> i64 {
    Line::new(b"power off with the wipe: ").str(why).end();
    let e = mk_admin_shutdown();
    Line::new(b"power off refused, errno ").dec(e.unsigned_abs()).end();
    e
}

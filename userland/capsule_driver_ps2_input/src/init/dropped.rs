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

//! The bytes the bring-up read from the port it was not waiting on and had
//! nowhere to deliver (read_port.rs says how they get there): before the
//! serve loop runs, a keystroke has no decode to go to. The count is all
//! that is kept, for the bring-up's log line.

use core::sync::atomic::{AtomicU32, Ordering};

static DROPPED: AtomicU32 = AtomicU32::new(0);

pub fn dropped() -> u32 {
    DROPPED.load(Ordering::Relaxed)
}

/// Drop a byte from the other port, counting it.
pub fn drop_byte(_: u8) {
    DROPPED.fetch_add(1, Ordering::Relaxed);
}

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

//! A master that writes and never reads: each ^C typed is echoed onto the
//! master's own output, so echo must stop when that output has no room, or
//! one write loop grows it until the personality runs out of memory.

use super::out::OUT_MOST;
use super::support::{table, EAGAIN};

const SHELL: u32 = 7;

#[test]
fn a_flood_of_ctrl_c_stops_at_the_masters_room() {
    let mut t = table(SHELL);
    let flood = alloc::vec![0x03u8; 64 << 10];
    let refused = (0..8).any(|_| t.typed_at(0, &flood) == Err(EAGAIN));
    assert!(refused, "a master that is never read stops taking bytes");
    let held = t.get(0).unwrap().queued(true);
    assert!(held <= OUT_MOST + 8, "{held} bytes held for the master");
}

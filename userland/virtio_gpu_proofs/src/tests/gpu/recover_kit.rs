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

//! What the recovery tests share: a whole flush, a flush the host stalls on
//! followed by the turn's settle, and the lines logged since a point.

use nonos_gfx_wire::{ops, Surface};
use nonos_libc::said;

use super::ask::{ask, send, words};
use super::rig::Rig;
use super::wrong::{run, Wrong};
use crate::server::recover::settle;

/// FLUSH of `s` whole: the status the compositor reads.
pub fn flush(r: &mut Rig, s: &Surface) -> i32 {
    ask(r, ops::FLUSH, 0, &words(&[s.serial, 1, 0, 0, s.width, s.height])).0
}

/// The lines the driver logged from line `from` on.
pub fn said_from(from: usize) -> Vec<String> {
    said().split_off(from)
}

/// A flush the host stalls on, from its next command for `d` (0: for good),
/// then the turn's settle: whether the driver went Down on it.
pub fn stall(r: &mut Rig, s: &Surface, d: u32) -> bool {
    let all = words(&[s.serial, 1, 0, 0, s.width, s.height]);
    let (status, _) = run(r, Wrong::Dead(1, d), |r| send(r, ops::FLUSH, 0, &all)).expect("alive");
    assert_ne!(status, 0, "a stalled flush is refused");
    settle(&mut r.d)
}

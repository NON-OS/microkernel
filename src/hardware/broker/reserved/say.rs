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

//! One serial line per refused grant, naming the range asked for and the
//! reservation it ran into, so a driver that stops working says why.

use super::store::conflict_with;
use super::wire::{Reservation, IO};
use crate::sys::serial::Line;

pub fn refused(grant: &[u8], base: u64, len: u64, r: &Reservation) {
    let space: &[u8] = if r.space == IO { b" io " } else { b" memory " };
    Line::new()
        .str(b"[BROKER] ")
        .str(grant)
        .str(b" refused: base=")
        .hex(base)
        .str(b" len=")
        .hex(len)
        .str(b" overlaps firmware-reserved")
        .str(space)
        .hex(r.base)
        .str(b"+")
        .hex(r.len)
        .end();
}

/// Whether the grant runs into a reservation, said on serial when it does.
pub fn refuses(grant: &[u8], space: u8, base: u64, len: u64, own_bar: bool) -> bool {
    let r = conflict_with(space, base, len, own_bar);
    r.inspect(|r| refused(grant, base, len, r)).is_some()
}

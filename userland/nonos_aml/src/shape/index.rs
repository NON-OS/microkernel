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

//! The opcode-to-row index, built from the rows when this crate compiles.

use super::data::DATA;
use super::expr::EXPRS;
use super::object::OBJECTS;
use super::statement::STATEMENTS;
use super::Group;
use crate::op::Op;

// Each row list holds fewer than 32 rows, so list * 32 + row + 1 fits a byte
// and 0 can mean "no row".
const LISTS: [&[Group]; 4] = [OBJECTS, STATEMENTS, EXPRS, DATA];

const fn build(page: u16) -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut l = 0;
    while l < LISTS.len() {
        let mut r = 0;
        while r < LISTS[l].len() {
            let ops = LISTS[l][r].ops;
            let mut i = 0;
            while i < ops.len() {
                if ops[i] >> 8 == page {
                    let slot = (ops[i] & 0xFF) as usize;
                    // An opcode in two rows would make the grammar ambiguous;
                    // refuse to compile rather than pick one.
                    assert!(table[slot] == 0, "opcode in two shape rows");
                    assert!(r < 32, "shape list longer than 32 rows");
                    table[slot] = (l * 32 + r + 1) as u8;
                }
                i += 1;
            }
            r += 1;
        }
        l += 1;
    }
    table
}

static PRIMARY: [u8; 256] = build(0);
static EXTENDED: [u8; 256] = build(Op::EXT_PREFIX as u16);

/// The row for an opcode, or None for one that has no operands to describe
/// (a constant, a string, a local) or that does not exist.
pub fn lookup(op: Op) -> Option<&'static Group> {
    let slot = match op.0 >> 8 {
        0 => PRIMARY[op.0 as usize],
        0x5B => EXTENDED[(op.0 & 0xFF) as usize],
        _ => 0,
    };
    let s = slot.checked_sub(1)? as usize;
    LISTS[s / 32].get(s % 32)
}

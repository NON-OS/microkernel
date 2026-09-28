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

//! Pulling one frame off a negotiated link, fixed or variable.

extern crate alloc;

use alloc::vec::Vec;

use super::super::commands::is_variable;
use super::super::fixed::Cell;
use super::super::geometry::{CELL_BYTES, VARIABLE_MAX};
use super::super::var::VarCell;

/// One frame read off a link.
pub enum Frame {
    Fixed(Cell),
    Var(VarCell),
}

/// Read one frame from the front of a link that has finished negotiating, so
/// every circuit id is four bytes wide.
pub fn parse(bytes: &[u8]) -> Option<(Frame, usize)> {
    if bytes.len() < 5 {
        return None;
    }
    let circuit = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let command = bytes[4];
    if !is_variable(command) {
        return Cell::decode(bytes).map(|cell| (Frame::Fixed(cell), CELL_BYTES));
    }
    if bytes.len() < 7 {
        return None;
    }
    let length = u16::from_be_bytes([bytes[5], bytes[6]]) as usize;
    if length > VARIABLE_MAX {
        return None;
    }
    let total = 7 + length;
    let body: Vec<u8> = bytes.get(7..total)?.to_vec();
    Some((Frame::Var(VarCell { circuit, command, body }), total))
}

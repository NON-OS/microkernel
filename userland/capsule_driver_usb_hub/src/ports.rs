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
//! A host's root ports as PORT_STATUS lists them: entries of kind 0, the
//! virtual ports of parked children (kind 1) left to the class drivers.

use alloc::vec::Vec;

use nonos_usb_hub::Root;

use crate::wire::{call, OP_PORT_STATUS};

/// A reply's head and 255 entries fit.
const LIST_MAX: usize = 4 + 8 * 255;

pub fn roots(host: u32) -> Option<Vec<Root>> {
    let mut data = [0u8; LIST_MAX];
    let (status, n) = call_list(host, &mut data);
    if status != 0 || n < 4 || n < 4 + 8 * data[0] as usize {
        return None;
    }
    let entries = data[4..4 + 8 * data[0] as usize].chunks_exact(8);
    let sc = |e: &[u8]| u32::from_le_bytes([e[4], e[5], e[6], e[7]]);
    Some(entries.filter(|e| e[2] == 0).map(|e| (e[0], e[1], sc(e))).collect())
}

fn call_list(host: u32, data: &mut [u8]) -> (i32, usize) {
    call(host, OP_PORT_STATUS, &[], data)
}

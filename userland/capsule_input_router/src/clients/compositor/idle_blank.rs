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

//! IDLE_BLANK to the compositor: the screen blank (true) or woken (false).
//! True when the compositor took it.

use super::super::wire::{call, lookup_port};
use super::constants::{MAGIC, OP_IDLE_BLANK, SERVICE};

pub fn idle_blank(port_slot: &mut u32, request_id: u32, blank: bool) -> bool {
    if *port_slot == 0 {
        match lookup_port(SERVICE) {
            Some(port) => *port_slot = port,
            None => return false,
        }
    }
    let payload = u32::from(blank).to_le_bytes();
    match call(*port_slot, MAGIC, OP_IDLE_BLANK, request_id, &payload, &mut []) {
        Ok(status) => status == 0,
        Err(_) => {
            *port_slot = 0;
            false
        }
    }
}

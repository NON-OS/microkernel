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

//! The bulk calls by pair: BOT's on the first pair alone, UAS's command
//! and status on the first and its data on the second (OP_CONFIGURE_BULK2).

use super::bulk::{bulk_in_on, bulk_out_on, configure_on, reset_bulk_on};
use super::wire::{OP_CONFIGURE_BULK, OP_CONFIGURE_BULK2};
use crate::descriptors::MscBinding;

/// The pair a UAS command's data moves on.
pub const DATA_PAIR: u8 = 1;

pub fn configure_bulk(xhci: u32, slot: u8, b: &MscBinding) -> Result<(), i32> {
    configure_on(xhci, slot, OP_CONFIGURE_BULK, b)
}

/// The second pair: a UAS device's data-in and data-out pipes.
pub fn configure_bulk2(xhci: u32, slot: u8, b: &MscBinding) -> Result<(), i32> {
    configure_on(xhci, slot, OP_CONFIGURE_BULK2, b)
}

pub fn bulk_out(xhci: u32, slot: u8, data: &[u8]) -> Result<usize, i32> {
    bulk_out_on(xhci, slot, 0, data)
}

pub fn bulk_in(xhci: u32, slot: u8, out: &mut [u8]) -> Result<usize, i32> {
    bulk_in_on(xhci, slot, 0, out)
}

pub fn reset_bulk(xhci: u32, slot: u8, dir_in: bool) -> Result<(), i32> {
    reset_bulk_on(xhci, slot, 0, dir_in)
}

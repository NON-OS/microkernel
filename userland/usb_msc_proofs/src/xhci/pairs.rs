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

//! The calls the driver's UAS transport makes: by pair, each recorded with
//! its pair and answered as the first pair's are; the second pair and
//! streams, configured; and the port's speed, high speed, where a UAS
//! interface would be bound.

use super::bulk::{bulk_in, bulk_out, reset_bulk};
use super::device::setup;
use super::window::call;
use crate::descriptors::MscBinding;

pub const DATA_PAIR: u8 = 1;
const SPEED_HIGH: u8 = 3;

pub fn bulk_out_on(xhci: u32, slot: u8, pair: u8, data: &[u8]) -> Result<usize, i32> {
    setup(|d| d.sent.push((pair, data.to_vec())));
    bulk_out(xhci, slot, data)
}

pub fn bulk_in_on(xhci: u32, slot: u8, pair: u8, out: &mut [u8]) -> Result<usize, i32> {
    setup(|d| d.read_on.push(pair));
    bulk_in(xhci, slot, out)
}

pub fn reset_bulk_on(xhci: u32, slot: u8, _pair: u8, dir_in: bool) -> Result<(), i32> {
    reset_bulk(xhci, slot, dir_in)
}

pub fn configure_bulk2(_xhci: u32, _slot: u8, _b: &MscBinding) -> Result<(), i32> {
    call()
}

pub fn configure_streams(_xhci: u32, _slot: u8, _pair: u8, _dirs: u8, n: u8) -> Result<u16, i32> {
    call().map(|_| (1u16 << n) - 1)
}

pub fn port_speed(_xhci: u32, _port: u8) -> Option<u8> {
    Some(SPEED_HIGH)
}

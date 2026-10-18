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
//! A shared device's descriptors read through the host's own read ops
//! (GET_DEVICE_DESCRIPTOR, GET_CONFIG_DESCRIPTOR), which serve a driver
//! that has not yet claimed an interface; a control transfer would not.
//! Only the first configuration: the one the sharing driver set.

use alloc::vec;

use super::call::call;
use super::wire::{CONTROL_MAX, DATA_AT, E_IO};
use super::wire::{OP_GET_CONFIG_DESCRIPTOR, OP_GET_DEVICE_DESCRIPTOR};
use crate::desc::device_info;
use crate::found::Found;

const DEVICE_LEN: usize = 18;
/// A configuration reply's length le16 and two bytes before its data.
const CONFIG_PREFIX: usize = 4;

pub fn read_shared(xhci: u32, slot: u8, port: u8) -> Result<Found, i32> {
    let mut resp = [0u8; DATA_AT + DEVICE_LEN];
    if call(xhci, OP_GET_DEVICE_DESCRIPTOR, &[slot], &mut resp)? < DEVICE_LEN {
        return Err(E_IO);
    }
    let info = device_info(&resp[DATA_AT..]).ok_or(E_IO)?;
    let want = (CONTROL_MAX as u16).to_le_bytes();
    let mut resp = vec![0u8; DATA_AT + CONFIG_PREFIX + CONTROL_MAX];
    let n = call(xhci, OP_GET_CONFIG_DESCRIPTOR, &[slot, 0, want[0], want[1]], &mut resp)?;
    let got = u16::from_le_bytes([resp[DATA_AT], resp[DATA_AT + 1]]) as usize;
    if n < CONFIG_PREFIX + got || got > CONTROL_MAX {
        return Err(E_IO);
    }
    let config = resp[DATA_AT + CONFIG_PREFIX..DATA_AT + CONFIG_PREFIX + got].to_vec();
    Ok(Found { port, info, configs: vec![config] })
}

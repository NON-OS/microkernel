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

//! ITS commands: four doublewords, the opcode in the low byte (IHI 0069,
//! 5.3). Only the physical-LPI commands; nothing here maps a vPE.

pub type Cmd = [u64; 4];

pub(super) const SYNC: u64 = 0x05;
const MAPD: u64 = 0x08;
pub(super) const MAPC: u64 = 0x09;
const MAPTI: u64 = 0x0a;
const INV: u64 = 0x0c;
const DISCARD: u64 = 0x0f;

pub(super) const VALID: u64 = 1 << 63;
const ADDR_51_8: u64 = 0x000f_ffff_ffff_ff00;

const fn device(op: u64, device_id: u32) -> u64 {
    op | (device_id as u64) << 32
}

/// Give `device_id` the interrupt translation table at `itt`, which holds
/// `1 << event_bits` entries. `event_bits` is at least one.
pub const fn mapd(device_id: u32, itt: u64, event_bits: u32) -> Cmd {
    let size = (event_bits.saturating_sub(1) & 0x1f) as u64;
    [device(MAPD, device_id), size, VALID | (itt & ADDR_51_8), 0]
}

/// Take the table away: the ITS then drops every event of the device.
pub const fn unmapd(device_id: u32) -> Cmd {
    [device(MAPD, device_id), 0, 0, 0]
}

/// Translate `event` of `device_id` to `lpi`, delivered to `collection`.
pub const fn mapti(device_id: u32, event: u32, lpi: u32, collection: u16) -> Cmd {
    [device(MAPTI, device_id), event as u64 | (lpi as u64) << 32, collection as u64, 0]
}

/// Reload the configuration byte of the LPI `event` translates to.
pub const fn inv(device_id: u32, event: u32) -> Cmd {
    [device(INV, device_id), event as u64, 0, 0]
}

/// Forget the translation and any interrupt it left pending.
pub const fn discard(device_id: u32, event: u32) -> Cmd {
    [device(DISCARD, device_id), event as u64, 0, 0]
}

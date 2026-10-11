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

//! The AUX channel's registers (SKL PRM Vol 2c, DP_AUX_CH_CTL and
//! DP_AUX_CH_DATA1-5): one control register and five data registers per
//! DDI, A at 0x64010, each further DDI 0x100 on. The data registers hold
//! the message's bytes most significant first, four to a register.

/// The DDIs with an AUX channel on Gen9: A (usually eDP) to D.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddi {
    A,
    B,
    C,
    D,
}

pub fn aux_ctl(d: Ddi) -> u32 {
    0x6_4010 + 0x100 * d as u32
}

pub fn aux_data(d: Ddi, i: u32) -> u32 {
    aux_ctl(d) + 4 + 4 * i
}

/// DP_AUX_CH_CTL fields. Send/Busy, Done, Time Out Error and Receive
/// Error with Message Size are the PRM's; the timer and sync pulse
/// encodings are unverified against the volume.
pub const SEND_BUSY: u32 = 1 << 31;
pub const DONE: u32 = 1 << 30;
pub const TIMEOUT_ERROR: u32 = 1 << 28;
/// Time Out Timer Value, 27:26: 11b, the longest, 1600 us.
pub const TIMEOUT_1600US: u32 = 3 << 26;
pub const RECEIVE_ERROR: u32 = 1 << 25;
pub const SIZE_SHIFT: u32 = 20;
pub const SIZE_MASK: u32 = 0x1F << 20;
/// AUX Sync Pulse Count, 9:5, stored less one: 32 pulses.
pub const SYNC_PULSES_32: u32 = 31 << 5;

/// The control word that sends `len` bytes, clearing the sticky status
/// bits (each written one to clear) in the same write.
pub fn send_word(len: usize) -> u32 {
    let status = DONE | TIMEOUT_ERROR | RECEIVE_ERROR;
    SEND_BUSY | status | TIMEOUT_1600US | ((len as u32) << SIZE_SHIFT) | SYNC_PULSES_32
}

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

//! The control family's tags, ops and sizes, and what a reply is drawn from.

use nonos_wifi_core::scan_list::{ScanResults, MAX_RESULTS};

/// The control family tag.
pub(super) const WIFI_MAGIC: u32 = 0x5749_4649;
pub(super) const WIFI_HDR: usize = 10;
/// net_core's link protocol tag (`netif::wire::MAGIC_NNET`).
pub(super) const NNET_MAGIC: u32 = 0x4E4E_4554;
pub const OP_CONNECT: u16 = 1;
pub const OP_DISCONNECT: u16 = 2;
pub const OP_LINK: u16 = 5;
pub(super) const OP_SCAN: u16 = 3;
pub(super) const OP_STATUS: u16 = 4;
/// ENOSYS: the op exists in the family and this driver does not run it.
pub(super) const E_NO_OP: i32 = -38;
pub(super) const SCAN_STATS: usize = 12;
pub(super) const STATUS_LEN: usize = 1 + 1 + 4 * 7;
pub(super) const SSID_MAX: usize = 32;

/// The longest reply: a scan with every network at the longest SSID.
pub const REPLY_MAX: usize = WIFI_HDR + SCAN_STATS + 1 + MAX_RESULTS * (3 + SSID_MAX);

/// What the radio has heard, while it is up.
pub struct Heard<'a> {
    pub list: &'a ScanResults,
    pub sweeps: u32,
    pub stalls: u32,
    pub frames: u32,
    pub beacons: u32,
}

/// What a control request is answered from.
pub struct View<'a> {
    pub stage: u8,
    pub step: u8,
    pub detail: u32,
    pub hw_rev: u32,
    pub rf_id: u32,
    /// `Some` while the firmware is up and scanning.
    pub heard: Option<Heard<'a>>,
}

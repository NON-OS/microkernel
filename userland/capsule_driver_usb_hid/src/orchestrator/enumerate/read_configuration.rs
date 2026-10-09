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

use crate::descriptors::{total_length, CONFIG_HEADER_LEN};
use crate::xhci::get_config_descriptor;

/// The whole configuration descriptor: its 9-byte header first, for
/// wTotalLength, then that many bytes (at most what one reply carries). A
/// fixed 64-byte read cut every longer configuration short, and the parser
/// rightly refused it: a wireless keyboard and mouse receiver or a gaming
/// keyboard, with three interfaces, is longer than that.
pub(super) fn read_configuration(xhci_port: u32, slot: u8, desc: &mut [u8]) -> Option<usize> {
    let n = get_config_descriptor(xhci_port, slot, CONFIG_HEADER_LEN, desc).ok()?;
    let total = total_length(&desc[..n])?;
    let want = total.clamp(CONFIG_HEADER_LEN, desc.len() as u16);
    get_config_descriptor(xhci_port, slot, want, desc).ok()
}

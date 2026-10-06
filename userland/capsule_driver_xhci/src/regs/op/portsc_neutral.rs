// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The value to write to PORTSC to change one thing and nothing else, as
//! Linux's `xhci_port_state_to_neutral` builds it. PORTSC mixes read-only
//! bits, RWS bits that must be written back as read (power, link state,
//! indicator, wake enables), and RW1C/RW1S bits where writing back a one
//! acts: a one in PED disables the port, a one in a change bit acknowledges
//! it, and a one in PR or WPR resets the port. The neutral value keeps the
//! first two kinds and zeroes the rest.

/// CCS, OCA, speed and DR: read only, ignored on write.
const PORT_RO: u32 = (1 << 0) | (1 << 3) | (0xF << 10) | (1 << 30);
/// PLS, PP, PIC and the three wake enables. PLS is written back only with
/// LWS clear, so the controller ignores it.
const PORT_RWS: u32 = (0xF << 5) | (1 << 9) | (0x3 << 14) | (0x7 << 25);

pub fn portsc_neutral(portsc: u32) -> u32 {
    portsc & (PORT_RO | PORT_RWS)
}

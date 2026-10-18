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

//! MCFG from the table's bytes, with no pointer read: after the 36-byte
//! header and eight reserved bytes, 16-byte allocations, each the ECAM
//! base u64, the segment group u16, the first and last bus u8 and four
//! reserved bytes (PCI Firmware 3.x, recalled: unverified). The declared
//! length is believed only as far as the bytes handed over; an inverted bus
//! range or a zero base reaches nothing and is skipped, and past
//! `MAX_WINDOWS` the rest are not read.

use alloc::vec::Vec;

pub(crate) const WINDOWS_AT: usize = 44;
pub(crate) const WINDOW_LEN: usize = 16;
/// Segment groups kept; a large server describes a handful.
pub(crate) const MAX_WINDOWS: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Window {
    pub base: u64,
    pub segment: u16,
    pub start_bus: u8,
    pub end_bus: u8,
}

pub(crate) fn windows(t: &[u8]) -> Vec<Window> {
    let Some(h) = t.get(4..8) else { return Vec::new() };
    let declared = u32::from_le_bytes([h[0], h[1], h[2], h[3]]) as usize;
    let t = &t[..declared.min(t.len())];
    let all = t.get(WINDOWS_AT..).unwrap_or(&[]).chunks_exact(WINDOW_LEN);
    let read = all.take(MAX_WINDOWS).map(|w| Window {
        base: u64::from_le_bytes([w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]]),
        segment: u16::from_le_bytes([w[8], w[9]]),
        start_bus: w[10],
        end_bus: w[11],
    });
    read.filter(|w| w.base != 0 && w.start_bus <= w.end_bus).collect()
}

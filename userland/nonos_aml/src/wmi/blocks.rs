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

//! A WMI mapper's _WDG (PNP0C14, Microsoft's published "WMI and ACPI"
//! mapping): a buffer of 20-byte blocks, each a GUID, two bytes that are a
//! method's object ID or an event's notification value, an instance count
//! and flags. An event block names what a Notify on the device carries,
//! and _WED(value) returns its data.

use alloc::vec::Vec;

pub const BLOCK: usize = 20;
pub const EXPENSIVE: u8 = 0x1;
pub const METHOD: u8 = 0x2;
pub const STRING: u8 = 0x4;
pub const EVENT: u8 = 0x8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WmiBlock {
    pub guid: [u8; 16],
    /// A method's or data block's two-character object ID; for an event,
    /// the first byte is the notification value.
    pub id: [u8; 2],
    pub instances: u8,
    pub flags: u8,
}

impl WmiBlock {
    pub fn notify_value(&self) -> Option<u8> {
        (self.flags & EVENT != 0).then_some(self.id[0])
    }
}

/// Every whole block; a trailing partial block is not read.
pub fn parse_wdg(bytes: &[u8]) -> Vec<WmiBlock> {
    let block = |c: &[u8]| WmiBlock {
        guid: c[..16].try_into().unwrap_or([0; 16]),
        id: [c[16], c[17]],
        instances: c[18],
        flags: c[19],
    };
    bytes.chunks_exact(BLOCK).map(block).collect()
}

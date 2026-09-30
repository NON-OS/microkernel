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

//! The one device this driver serves, once it is bound.

use crate::descriptors::MscBinding;

/// A bound BOT device: where driver.xhci0 keeps it and what it reported.
#[derive(Clone, Copy)]
pub struct Disk {
    pub xhci: u32,
    pub slot: u8,
    pub interface: u8,
    pub ep_in: u8,
    pub ep_out: u8,
    /// Logical blocks READ CAPACITY(10) reported, at most 2^32.
    pub blocks: u64,
    /// The device's logical block size; only 512 is served.
    pub block_len: u32,
}

impl Disk {
    /// The device on `slot` bound through `b`, before its size is asked.
    pub fn bound(xhci: u32, slot: u8, b: &MscBinding) -> Self {
        let (interface, ep_in, ep_out) = (b.interface, b.bulk_in, b.bulk_out);
        Self { xhci, slot, interface, ep_in, ep_out, blocks: 0, block_len: 0 }
    }
}

/// The data phase of one command.
pub enum Data<'a> {
    None,
    In(&'a mut [u8]),
    Out(&'a [u8]),
}

impl Data<'_> {
    pub fn len(&self) -> usize {
        match self {
            Data::None => 0,
            Data::In(b) => b.len(),
            Data::Out(b) => b.len(),
        }
    }
}

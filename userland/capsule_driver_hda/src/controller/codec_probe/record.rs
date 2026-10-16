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

//! What the probe found at one codec address.

pub const MAX_CODECS: usize = 15;

#[derive(Clone, Copy)]
pub struct CodecProbe {
    pub address: u8,
    pub present: u8,
    pub ok: u8,
    pub vendor_id: u16,
    pub device_id: u16,
}

pub(super) fn decode(address: u8, id: u32) -> CodecProbe {
    CodecProbe { address, present: 1, ok: 1, vendor_id: (id >> 16) as u16, device_id: id as u16 }
}

pub(super) const fn empty() -> CodecProbe {
    CodecProbe { address: 0, present: 0, ok: 0, vendor_id: 0, device_id: 0 }
}

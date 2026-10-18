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

//! GPIO_EVENT's LIST body: count u16, then per pin: pin u16, flags u8 (bit
//! 0 edge, bits 2:1 polarity, bit 3 shared, bit 4 wake), debounce u16 in
//! hundredths of a millisecond; at most `LISTED` pins, which fill a reply.

use alloc::vec::Vec;
use nonos_aml::aei::AeiPin;

/// (512 - status 4 - count 2) / 5.
pub const LISTED: usize = 101;

pub fn list(pins: &[AeiPin]) -> Vec<u8> {
    let mut b = (pins.len().min(LISTED) as u16).to_le_bytes().to_vec();
    for p in pins.iter().take(LISTED) {
        let flags = u8::from(p.edge) | (p.polarity & 3) << 1 | u8::from(p.shared) << 3;
        b.extend_from_slice(&p.pin.to_le_bytes());
        b.push(flags | u8::from(p.wake) << 4);
        b.extend_from_slice(&p.debounce_centi_ms.to_le_bytes());
    }
    b
}

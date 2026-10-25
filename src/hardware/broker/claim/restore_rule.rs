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

//! What a claimed function's BAR registers must read once it is in D0 (L62). A
//! move to D3cold, to D3hot without No_Soft_Reset, or a reset leaves its BARs at
//! zero and decoding off, so the windows granted no longer answer. Pure.

/// A BAR as enumeration decoded it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Kind {
    Absent,
    Mem32,
    Mem64,
    Io,
}

/// What the register at a BAR's slot must hold: the address bits, which a
/// read is compared under `mask`; the type bits below them are the device's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Want {
    pub value: u32,
    pub mask: u32,
}

/// The low register and, for a 64-bit BAR, the high one at the next slot.
pub(crate) const fn wants(kind: Kind, address: u64) -> (Option<Want>, Option<Want>) {
    let low = address as u32;
    match kind {
        Kind::Absent => (None, None),
        Kind::Mem32 => (Some(Want { value: low & !0xF, mask: !0xF }), None),
        Kind::Mem64 => {
            let high = Want { value: (address >> 32) as u32, mask: u32::MAX };
            (Some(Want { value: low & !0xF, mask: !0xF }), Some(high))
        }
        Kind::Io => (Some(Want { value: low & !0x3, mask: !0x3 }), None),
    }
}

/// Each slot's want from the six BARs as enumeration decoded them, a 64-bit
/// BAR's high half in the next slot (the one enumeration marked absent).
pub(crate) fn slots(bars: [(Kind, u64); 6]) -> [Option<Want>; 6] {
    let mut slots = [None; 6];
    for (i, (kind, address)) in bars.into_iter().enumerate() {
        let (low, high) = wants(kind, address);
        slots[i] = low.or(slots[i]);
        if let (Some(high), Some(next)) = (high, slots.get_mut(i + 1)) {
            *next = Some(high);
        }
    }
    slots
}

/// What to write so the register holds `want`: its address bits, the device's
/// read-only type bits below them as read.
pub(crate) const fn put(read: u32, want: Want) -> u32 {
    (want.value & want.mask) | (read & !want.mask)
}

/// Whether `read` already holds `want`.
pub(crate) const fn holds(read: u32, want: Want) -> bool {
    read & want.mask == want.value & want.mask
}

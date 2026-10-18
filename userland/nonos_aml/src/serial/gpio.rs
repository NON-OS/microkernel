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

//! GPIO Connection descriptors (ACPI 6.5, 6.4.3.8.1): GpioInt, a pin that
//! interrupts, and GpioIo, pins driven or read. Their pin table, source
//! name and vendor data sit at offsets the descriptor gives, counted from
//! its first byte; an offset outside the descriptor skips it.

use super::le::u16_at as le16;
use crate::platform::items;
use alloc::vec::Vec;

const GPIO: u8 = 0x0C;
const MIN: usize = 23;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gpio {
    /// GpioInt, else GpioIo; then, for GpioInt, edge rather than level.
    pub interrupt: bool,
    pub edge: bool,
    /// GpioInt: 0 active high, 1 active low, 2 both edges. GpioIo: the
    /// IO restriction (0 none, 1 input, 2 output, 3 preserve).
    pub polarity: u8,
    pub shared: bool,
    /// GpioInt: the pin can wake the system.
    pub wake: bool,
    /// 0 default, 1 pull-up, 2 pull-down, 3 none; higher is the vendor's.
    pub pull: u8,
    pub debounce_centi_ms: u16,
    pub pins: Vec<u16>,
    /// The GPIO controller's path as text.
    pub source: Vec<u8>,
}

fn gpio(w: &[u8]) -> Option<Gpio> {
    if w.len() < MIN {
        return None;
    }
    let u16_at = |w: &[u8], i: usize| le16(w, i) as usize;
    let (flags, pin_at, name_at, vendor_at) =
        (u16_at(w, 7), u16_at(w, 14), u16_at(w, 17), u16_at(w, 19));
    let name_end = if vendor_at > name_at && u16_at(w, 21) > 0 { vendor_at } else { w.len() };
    let pins = w.get(pin_at..name_at)?.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]]));
    let interrupt = w[4] == 0;
    Some(Gpio {
        interrupt,
        edge: interrupt && flags & 1 != 0,
        polarity: if interrupt { ((flags >> 1) & 3) as u8 } else { (flags & 3) as u8 },
        shared: flags & 8 != 0,
        wake: interrupt && flags & 16 != 0,
        pull: w[9],
        debounce_centi_ms: u16_at(w, 12) as u16,
        pins: pins.collect(),
        source: w.get(name_at..name_end)?.iter().copied().take_while(|c| *c != 0).collect(),
    })
}

pub fn gpios(t: &[u8]) -> Vec<Gpio> {
    let ours = |i: &&crate::platform::Item<'_>| i.large && i.kind == GPIO;
    items(t).iter().filter(ours).filter_map(|i| gpio(i.whole)).collect()
}

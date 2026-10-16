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

//! The functions behind a port: probed on its secondary bus when a card
//! arrives (device 0, the only one a port's link carries; each of its
//! functions when it has several), and every one on the buses below it
//! taken out of the kernel's table when it leaves, in eject mode for an
//! eject, which puts back what it took if a driver still holds one.

use super::find::Bdf;
use super::walk::read;
use crate::sys::pci_depart::{depart, Why, EBUSY};
use crate::sys::pci_probe::probe_function;
use alloc::vec::Vec;

const SECONDARY: u16 = 0x19;
const SUBORDINATE: u16 = 0x1A;

fn below(f: Bdf) -> impl Iterator<Item = Bdf> {
    let (lo, hi) = (read(f, SECONDARY, 1) as u8, read(f, SUBORDINATE, 1) as u8);
    // A port with no secondary bus set leads nowhere.
    let set = lo != 0 && lo != 0xFF;
    (lo..=hi.max(lo))
        .filter(move |_| set)
        .flat_map(move |b| (0..32u8).flat_map(move |d| (0..8u8).map(move |x| (f.0, b, d, x))))
}

pub fn probe(f: Bdf) -> usize {
    let bus = read(f, SECONDARY, 1) as u8;
    if bus == 0 || bus == 0xFF {
        return 0;
    }
    let funcs = if read((f.0, bus, 0, 0), 0x0E, 1) & 0x80 != 0 { 8 } else { 1 };
    (0..funcs).filter(|x| probe_function(bus, 0, *x).is_ok()).count()
}

pub fn eject(f: Bdf) -> Result<usize, ()> {
    let mut taken: Vec<Bdf> = Vec::new();
    for g in below(f) {
        match depart(g.1, g.2, g.3, Why::Eject) {
            Ok(_) => taken.push(g),
            Err(EBUSY) => {
                for t in &taken {
                    let _ = probe_function(t.1, t.2, t.3);
                }
                return Err(());
            }
            Err(_) => {}
        }
    }
    Ok(taken.len())
}

pub fn gone(f: Bdf) -> usize {
    below(f).filter(|g| depart(g.1, g.2, g.3, Why::Gone).is_ok()).count()
}

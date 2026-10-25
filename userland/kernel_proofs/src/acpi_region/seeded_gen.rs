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

//! Seeded machines for the region rule: maps of every kind, windows,
//! ports and functions other capsules hold, and the kernel's hardware.

use super::machine::Machine;
use super::rule::{Hardware, MemKind};

pub struct Rng(pub u64);

impl Rng {
    pub fn below(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n.max(1)
    }
    pub fn span(&mut self, top: u64, unit: u64) -> (u64, u64) {
        let lo = self.below(top / unit) * unit;
        (lo, lo + unit * (1 + self.below(16)))
    }
}

use Hardware as H;
use MemKind as K;
const KINDS: [MemKind; 7] =
    [K::Ram, K::Reserved, K::AcpiNvs, K::AcpiReclaim, K::EfiRuntime, K::Unmapped, K::Other];
const HARDWARE: [Hardware; 6] = [H::LocalApic, H::IoApic, H::Hpet, H::Iommu, H::Ecam, H::Msix];
pub const TOP: u64 = 0x40_0000;

pub fn machine(r: &mut Rng) -> Machine {
    let mut m =
        Machine { map: vec![], windows: vec![], ports: vec![], hardware: vec![], pci: vec![], reset: vec![] };
    for _ in 0..1 + r.below(8) {
        let (lo, hi) = r.span(TOP, 0x1000);
        m.map.push((lo, hi, KINDS[r.below(7) as usize]));
    }
    for _ in 0..r.below(4) {
        m.windows.push(r.span(TOP, 0x1000));
        let (lo, hi) = r.span(TOP, 0x100);
        m.hardware.push((lo, hi, HARDWARE[r.below(6) as usize]));
        let (lo, hi) = r.span(0x1000, 2);
        m.ports.push((lo as u16, hi as u16));
        m.pci.push((0, r.below(4) as u8, r.below(32) as u8, r.below(8) as u8));
    }
    m
}

/// Half the asks start a few pages below something the machine has.
pub fn near(r: &mut Rng, m: &Machine) -> (u64, u64) {
    let (base, end) = r.span(TOP, 0x1000);
    let edges: Vec<u64> = m.map.iter().map(|e| e.0).chain(m.hardware.iter().map(|h| h.0)).collect();
    match r.below(2) {
        0 => (base, end),
        _ => {
            let at = (edges[r.below(edges.len() as u64) as usize] & !0xFFF)
                .saturating_sub(0x1000 * r.below(4));
            (at, at + (end - base))
        }
    }
}

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

//! The GIC CPU Interface structure (type 0x0B, ACPI 6.5, 5.2.12.14): 76
//! bytes in ACPI 5.1, 80 from 6.0 (efficiency class, SPE overflow
//! interrupt), 82 in 6.5 (TRBE interrupt). The later fields are read only
//! when the structure is long enough to hold them.

use super::super::le::{u16_at, u32_at, u64_at, u8_at};

pub(crate) const MIN: usize = 76;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gicc {
    pub cpu_interface: u32,
    pub uid: u32,
    pub flags: u32,
    pub parking_version: u32,
    pub perf_gsiv: u32,
    pub parked_address: u64,
    pub base: u64,
    pub gicv: u64,
    pub gich: u64,
    pub vgic_gsiv: u32,
    pub gicr_base: u64,
    pub mpidr: u64,
    pub efficiency: Option<u8>,
    pub spe_gsiv: Option<u16>,
    pub trbe_gsiv: Option<u16>,
}

impl Gicc {
    pub fn enabled(&self) -> bool {
        self.flags & 1 != 0
    }
    /// Not enabled now, but may be brought online (ACPI 6.5).
    pub fn online_capable(&self) -> bool {
        self.flags & (1 << 3) != 0
    }

    /// `s` is the whole structure, its length already checked.
    pub fn read(s: &[u8]) -> Option<Gicc> {
        let long = |n: usize| s.len() >= n;
        Some(Gicc {
            cpu_interface: u32_at(s, 4)?,
            uid: u32_at(s, 8)?,
            flags: u32_at(s, 12)?,
            parking_version: u32_at(s, 16)?,
            perf_gsiv: u32_at(s, 20)?,
            parked_address: u64_at(s, 24)?,
            base: u64_at(s, 32)?,
            gicv: u64_at(s, 40)?,
            gich: u64_at(s, 48)?,
            vgic_gsiv: u32_at(s, 56)?,
            gicr_base: u64_at(s, 60)?,
            mpidr: u64_at(s, 68)?,
            efficiency: if long(80) { u8_at(s, 76) } else { None },
            spe_gsiv: if long(80) { u16_at(s, 78) } else { None },
            trbe_gsiv: if long(82) { u16_at(s, 80) } else { None },
        })
    }
}

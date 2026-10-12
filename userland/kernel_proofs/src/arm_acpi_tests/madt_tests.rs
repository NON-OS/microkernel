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

//! MADT's GIC structures read field by field, in each revision's length,
//! and every structure that does not fit refused with its offset.

use super::build::*;
use super::madt_build::*;
use crate::arm_acpi::madt::madt_gic;
use crate::arm_acpi::sdt::TableError;

#[test]
fn the_four_gic_structures() {
    let gicd = structure(
        0x0C,
        &[vec![0, 0], u32le(0), u64le(0x2F00_0000), u32le(0), vec![3, 0, 0, 0]].concat(),
    );
    let gicr = structure(0x0E, &[vec![0, 0], u64le(0x2F10_0000), u32le(0x20_0000)].concat());
    let its = structure(0x0F, &[vec![0, 0], u32le(7), u64le(0x2F02_0000), u32le(0)].concat());
    let other = structure(0x00, &[0; 6]);
    let t =
        madt(&[gicc(82, 0, 0x8000_0000, 1), gicc(76, 1, 0x8000_0001, 8), gicd, gicr, its, other]);
    let g = madt_gic(&t).unwrap();
    assert_eq!(g.gicc.len(), 2);
    let (a, b) = (g.gicc[0], g.gicc[1]);
    assert!(a.enabled() && !a.online_capable() && !b.enabled() && b.online_capable());
    assert_eq!((a.uid, a.mpidr, a.perf_gsiv, a.vgic_gsiv), (0, 0x8000_0000, 23, 25));
    assert_eq!((a.base, a.gicr_base), (0x2C00_0000, 0x2D00_0000));
    assert_eq!((a.efficiency, a.spe_gsiv, a.trbe_gsiv), (Some(1), Some(21), Some(22)));
    assert_eq!((b.efficiency, b.spe_gsiv, b.trbe_gsiv), (None, None, None), "ACPI 5.1's 76 bytes");
    assert_eq!(gicc(80, 2, 2, 1).len(), 80);
    assert_eq!((g.gicd[0].base, g.gicd[0].version), (0x2F00_0000, 3));
    assert_eq!((g.gicr[0].base, g.gicr[0].length), (0x2F10_0000, 0x20_0000));
    assert_eq!((g.its[0].id, g.its[0].base), (7, 0x2F02_0000));
}

#[test]
fn structures_that_do_not_fit() {
    let first = 44;
    let short_gicc = structure(0x0B, &[0; 70]);
    assert_eq!(madt_gic(&madt(&[short_gicc])), Err(TableError::Structure(first)), "under 76");
    let zero = vec![0x0F, 0];
    assert_eq!(madt_gic(&madt(&[zero])), Err(TableError::Structure(first)), "length 0");
    let mut past = structure(0x0E, &[0; 14]);
    past[1] = 40;
    assert_eq!(madt_gic(&madt(&[past])), Err(TableError::Structure(first)), "past the end");
    let gicd_short = structure(0x0C, &[0; 18]);
    let ok = gicc(80, 0, 0, 1);
    assert_eq!(madt_gic(&madt(&[ok, gicd_short])), Err(TableError::Structure(first + 80)));
    let mut t = madt(&[]);
    t[0] = b'X';
    assert_eq!(madt_gic(&t), Err(TableError::Signature));
    let mut t = madt(&[]);
    t[20] ^= 1;
    assert_eq!(madt_gic(&t), Err(TableError::Checksum));
    assert_eq!(madt_gic(&madt(&[])[..40]), Err(TableError::Length));
}

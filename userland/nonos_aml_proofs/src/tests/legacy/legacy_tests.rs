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

//! Legacy devices from the namespace with their evaluated resources, the
//! FADT's boot flags, and the 8042 verdict.

use super::build::{fixed_io, io, mem32};
use super::rig::machine;
use crate::consts::at;
use crate::legacy::*;
use crate::platform::inventory;
use crate::probe::Probe;
use crate::run::boot;
use crate::tests::platform::run::evaluator;

fn found(with_ps2: bool) -> (crate::load::Aml, Vec<Legacy>) {
    let (mut aml, _) = boot(&machine(with_ps2), 2);
    let mut probe = Probe::default();
    let (devices, _) = inventory(&mut aml, &mut evaluator(&mut probe));
    let l = legacy_devices(&mut aml, &mut evaluator(&mut probe), &devices);
    (aml, l)
}

#[test]
fn devices_with_their_evaluated_resources() {
    let (aml, l) = found(true);
    let kinds: Vec<LegacyKind> = l.iter().map(|d| d.kind).collect();
    use LegacyKind::*;
    assert_eq!(kinds, [Keyboard, Mouse, Uart, Rtc, Hpet], "the PIT says absent");
    let r = |base, len| Range { base, len };
    assert_eq!(l[0].io, [r(0x60, 1), r(0x64, 1)]);
    assert_eq!(l[0].irqs.iter().map(|i| i.gsiv).collect::<Vec<_>>(), [1]);
    assert_eq!((l[1].irqs[0].gsiv, l[1].id), (12, at(&aml, "\\_SB.LPCB.PS2M").unwrap()), "by _CID");
    assert_eq!((l[2].io.as_slice(), l[2].uid), (&[r(0x3F8, 8)][..], Some(1)));
    assert_eq!(l[3].io, [r(0x70, 2)]);
    assert_eq!(l[4].memory, [r(0xFED0_0000, 0x400)]);
}

#[test]
fn the_8042_verdict() {
    let (_, with) = found(true);
    let (_, without) = found(false);
    let flag = |on: bool| Some(BootArch { has_8042: on, ..BootArch::default() });
    assert!(has_8042(flag(false), &with), "declared, though the FADT bit is clear");
    assert!(!has_8042(flag(false), &without), "neither: no 8042, and 0x60 is left alone");
    assert!(!has_8042(None, &without));
    assert!(has_8042(flag(true), &without), "the FADT alone");
}

#[test]
fn boot_flags_and_ranges() {
    let mut fadt = vec![0u8; 116];
    fadt[109] = 0b10_0011;
    let b = boot_arch(&fadt).unwrap();
    assert_eq!((b.legacy_devices, b.has_8042, b.no_vga, b.no_cmos_rtc), (true, true, false, true));
    assert_eq!(boot_arch(&fadt[..110]), None, "revision 1 is too short");
    let t = [fixed_io(0x4F8, 8), io(0x2F8, 0), mem32(0xFED0_0000, 0)[..6].to_vec(), vec![0x79, 0]]
        .concat();
    assert_eq!(io_ranges(&t), [Range { base: 0xF8, len: 8 }], "ten bits; zero length dropped");
    assert!(memory_ranges(&t).is_empty(), "a cut descriptor stops the walk");
}

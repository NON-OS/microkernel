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

//! F-101, read only: the FADT's flag by its bit inside the table; the known
//! UUIDs in a _DSM body's bytes; the LPS0 device absent, bare or with _DSM;
//! \_S3 by name. `lps0_seen` has no evaluator, so it cannot call the _DSM.

use super::idle_rig::pep;
use crate::enc::*;
use crate::platform::inventory;
use crate::rig::Rig;
use crate::suspend::*;

fn seen(r: &mut Rig) -> Lps0Seen {
    let Rig { aml, probe } = r;
    let (devices, _) = inventory(aml, &mut crate::tests::platform::run::evaluator(probe));
    lps0_seen(aml, &devices)
}

#[test]
fn the_fadt_flag_by_its_bit_and_only_inside_the_table() {
    let fadt = |flags: u32, len: usize| {
        let t = [vec![0u8; 112], flags.to_le_bytes().to_vec(), vec![0; 160]].concat();
        t[..len].to_vec()
    };
    assert!(fadt_s0_idle(&fadt(1 << 21, 116)) && !fadt_s0_idle(&fadt(1 << 20, 276)));
    assert!(fadt_s0_idle(&fadt(LOW_POWER_S0_IDLE | 1 << 20, 276)));
    assert!(!fadt_s0_idle(&fadt(!LOW_POWER_S0_IDLE, 276)), "every other bit set");
    assert!(!fadt_s0_idle(&fadt(LOW_POWER_S0_IDLE, 115)), "cut before the flags end");
    assert!(!fadt_s0_idle(&[]));
}

#[test]
fn uuids_are_found_whole_anywhere_in_the_body() {
    let all = [&[1u8, 2][..], &AMD_UUID, &[3], &INTEL_UUID, &MICROSOFT_UUID].concat();
    assert_eq!(uuids(&all), Uuids { microsoft: true, intel: true, amd: true });
    assert_eq!(uuids(&INTEL_UUID), Uuids { intel: true, ..Uuids::default() });
    assert_eq!(uuids(&MICROSOFT_UUID[..15]), Uuids::default(), "one byte short");
    assert_eq!(uuids(&[0x5A; 64]), Uuids::default());
}

#[test]
fn the_lps0_device_absent_bare_or_with_its_dsm() {
    let both = Uuids { microsoft: true, intel: true, amd: false };
    assert_eq!(seen(&mut pep(&[0x01], &[0x01])), Lps0Seen::Dsm(both));
    let bare = device("\\_SB.PEPD", &name_obj("_HID", &int(0x800D_D041)));
    assert_eq!(seen(&mut Rig::new(&bare).connected()), Lps0Seen::NoDsm);
    let gone = cat(&[&name_obj("_HID", &int(0x800D_D041)), &method("_STA", 0, &ret(&int(0)))]);
    let gone = cat(&[&device("\\_SB.PEPD", &gone), &method("\\_SB.PEPD._DSM", 4, &[])]);
    assert_eq!(seen(&mut Rig::new(&gone).connected()), Lps0Seen::Absent, "_STA says absent");
    let named = cat(&[&bare, &name_obj("\\_SB.PEPD._DSM", &int(0))]);
    assert_eq!(seen(&mut Rig::new(&named).connected()), Lps0Seen::Dsm(Uuids::default()));
}

#[test]
fn offered_by_the_flag_or_a_known_uuid_and_s3_by_its_name() {
    let intel = Lps0Seen::Dsm(Uuids { intel: true, ..Uuids::default() });
    assert!(offered(true, Lps0Seen::Absent) && offered(false, intel));
    assert!(!offered(false, Lps0Seen::Dsm(Uuids::default())) && !offered(false, Lps0Seen::NoDsm));
    let s3 = name_obj("\\_S3_", &package(4, &[int(5), int(5), int(0), int(0)]));
    assert!(has_s3(&Rig::new(&s3).aml) && !has_s3(&Rig::new(&[]).aml));
}

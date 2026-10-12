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

//! The inventory: EISA and string ids, _CID lists, _UID, _ADR and _STA.

use super::platform_run::evaluator;
use crate::consts::at;
use crate::enc::*;
use crate::platform::{eisa_id, inventory, HardwareId, DEFAULT};
use crate::probe::Probe;
use crate::run::boot;

#[test]
fn eisa_ids_decode() {
    assert_eq!(&eisa_id(0x030A_D041), b"PNP0A03");
    assert_eq!(&eisa_id(0x090C_D041), b"PNP0C09");
    assert_eq!(&eisa_id(0x0B0C_D041), b"PNP0C0B");
}

#[test]
fn devices_with_their_ids() {
    let ec = cat(&[&name_obj("_HID", &int(0x090C_D041)), &name_obj("_UID", &int(1))]);
    let ac = cat(&[
        &name_obj("_HID", &string("ACPI0003")),
        &name_obj("_CID", &package(2, &[string("PNP0C0A"), int(0x030A_D041)])),
        &name_obj("_STA", &int(0x1F)),
    ]);
    let src = cat(&[
        &device(
            "\\_SB.PCI0",
            &cat(&[
                &name_obj("_ADR", &int(0)),
                &device("LPCB", &cat(&[&name_obj("_ADR", &int(0x001F_0000)), &device("EC0", &ec)])),
            ]),
        ),
        &device("\\_SB.ADP1", &ac),
    ]);
    let (mut aml, _) = boot(&src, 2);
    let mut probe = Probe::default();
    let (devs, failed) = inventory(&mut aml, &mut evaluator(&mut probe));
    assert!(failed.is_empty());
    let ids: Vec<_> = devs.iter().map(|d| d.id).collect();
    let want = ["\\_SB.PCI0", "\\_SB.PCI0.LPCB", "\\_SB.PCI0.LPCB.EC0", "\\_SB.ADP1"];
    assert_eq!(ids, want.map(|p| at(&aml, p).unwrap()));
    assert_eq!(devs[1].adr, Some(0x001F_0000));
    assert_eq!(devs[2].hid, Some(HardwareId::Eisa(*b"PNP0C09")));
    assert_eq!((devs[2].uid, devs[2].sta), (Some(1), DEFAULT));
    assert_eq!(devs[3].hid, Some(HardwareId::Text(b"ACPI0003".to_vec())));
    assert_eq!(
        devs[3].cids,
        [HardwareId::Text(b"PNP0C0A".to_vec()), HardwareId::Eisa(*b"PNP0A03")]
    );
    assert_eq!(devs[3].sta, 0x1F);
}

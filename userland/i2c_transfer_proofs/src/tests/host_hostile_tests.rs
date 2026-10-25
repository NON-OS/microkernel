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

//! Records that name another controller than their "I2Cn" does, against
//! setup's pairing. The pci_progif of each says "I2C0", the bench function's
//! own name, but the kernel resolved its ResourceSource to another
//! controller's id, or to an id no controller has. The resolved id wins:
//! here the pad is not in reach, not swept for and no reason to try this
//! controller first; the controller its id names takes it.

use nonos_i2cmodel::{Bus, Config, Designware};
use nonos_libc::DeviceRecord;

use super::declared::{pad_at, reached};
use super::listed::{discovered, list, order, table};
use super::multi_sku::named;
use super::records::{hid, lpss, platform, I2C0};
use crate::bench::{settled_on, GEMINI_LAKE_HZ as HZ};
use crate::regs::{self, Attached};
use crate::transaction::Presence;

/// A core with a pad at 0x15, where a sweep on this bus would find it.
fn core() -> Attached {
    regs::attach(Designware::new(Config::LPSS, Bus::default().with(pad_at(0x15))))
}

#[test]
fn a_record_naming_another_controllers_id_is_not_paired_here_whatever_its_name() {
    let _core = core();
    let honest = hid(1, I2C0, 0x2A);
    let hostile = [hid(2, I2C0, 0), hid(2, I2C0, 0x2C), hid(0xDEAD, I2C0, 0x10)];
    list(&table(&[lpss(1), platform(2)], &[honest, hostile[0], hostile[1], hostile[2]]), None);
    let (cands, pads) = discovered();
    let d = settled_on(&cands[0], HZ, &pads[0], Presence::Absent, &pads).expect("bind");
    assert_eq!(named(&d, 3), [0x2A; 3], "only the pad that names this controller");
    assert!(regs::with(|c| c.bus().trace().is_empty()), "swept for another's pad");
    assert_eq!(order(&[cands[0], cands[1]], &pads[1..]), [1, 0], "tried first for another's");
    let t = settled_on(&cands[1], HZ, &pads[2], Presence::Absent, &pads).expect("bind");
    assert_eq!(reached(&t.reach.get()), [0x15, 0x2C], "its own pad, and the one swept for");
}

#[test]
fn a_resolved_host_wins_over_an_i2cn_naming_another_pci_function() {
    let _core = core();
    // Gemini Lake's I2C1 (8086:31AE) at id 2; the pads' name says I2C0, id 1.
    let i2c1 = DeviceRecord { device: 0x31AE, ..lpss(2) };
    list(&table(&[lpss(1), i2c1], &[hid(2, I2C0, 0x2C), hid(2, I2C0, 0)]), None);
    let (cands, pads) = discovered();
    assert_eq!(order(&cands, &pads), [1, 0], "tried first by the name, not the id");
    let named_by_name = settled_on(&cands[0], HZ, &pads[0], Presence::Absent, &pads).expect("bind");
    assert_eq!(named_by_name.bound_addr, 0x2C);
    assert!(regs::with(|c| c.bus().trace().is_empty()), "swept by the name");
    let host = settled_on(&cands[1], HZ, &pads[0], Presence::Absent, &pads).expect("bind");
    assert_eq!(reached(&host.reach.get()), [0x15, 0x2C], "its id's controller takes both");
}

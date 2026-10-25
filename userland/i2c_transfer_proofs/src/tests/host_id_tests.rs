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

//! The ids at the ends of the range, read from the device list by the
//! shipping discovery and paired by setup: a PCI id of 0 names its
//! controller like any other id, and u64::MAX, the kernel's word for a
//! ResourceSource it resolved to no controller, names none by id, not even
//! a controller listed with that id.

use nonos_i2cmodel::{Bus, Config, Designware};

use super::declared::{pad_at, reached};
use super::listed::{discovered, list, table};
use super::records::{hid, lpss};
use crate::bench::{settled_on, GEMINI_LAKE_HZ as HZ};
use crate::regs::{self, Attached};
use crate::transaction::Presence;

/// A bench core with a pad at 0x15, where the sweep reads first.
fn core() -> Attached {
    regs::attach(Designware::new(Config::LPSS, Bus::default().with(pad_at(0x15))))
}

#[test]
fn a_pci_id_of_0_names_its_controller_and_u64_max_names_none() {
    let sweepable = core();
    list(&table(&[lpss(0)], &[hid(0, 0, 0), hid(u64::MAX, 0, 0x2C)]), None);
    let (cands, pads) = discovered();
    let d = settled_on(&cands[0], HZ, &pads[0], Presence::Absent, &pads).expect("bind");
    assert_eq!(d.bound_addr, 0x15, "the pad declared on id 0 was swept for there");
    assert_eq!(reached(&d.reach.get()), [0x15, 0x2C], "an unresolved pad stays in reach");
    drop(sweepable);
    // u64::MAX is never a match, even for a controller listed with that id.
    let _core = core();
    list(&table(&[lpss(u64::MAX)], &[hid(u64::MAX, 0, 0)]), None);
    let (cands, pads) = discovered();
    let d = settled_on(&cands[0], HZ, &pads[0], Presence::Absent, &pads).expect("bind");
    assert!(regs::with(|c| c.bus().trace().is_empty()), "swept for a pad with no host");
    assert_eq!(d.bound_addr, 0);
}

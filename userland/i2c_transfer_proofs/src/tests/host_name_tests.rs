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

//! A pad the kernel left unresolved (u64::MAX), named only by its "I2Cn".
//! The kernel resolves a platform host by its ACPI window, and a PCI LPSS
//! function, declared by _ADR under PCI0, has none; Intel firmware names
//! those functions I2Cn in PCI function order, so a PCI function is paired
//! by that name, as every PCI host was before hosts were named by id. A
//! platform controller never is, and a pad with no "I2Cn" pairs with none.

use nonos_i2cmodel::{Bus, Config, Designware};
use nonos_libc::DeviceRecord;

use super::declared::{pad_at, reached};
use super::listed::{discovered, list, order, table};
use super::records::{hid, lpss, platform, I2C0};
use crate::bench::{settled_on, GEMINI_LAKE_HZ as HZ};
use crate::regs::{self, Attached};
use crate::transaction::Presence;

fn core() -> Attached {
    regs::attach(Designware::new(Config::LPSS, Bus::default().with(pad_at(0x15))))
}

#[test]
fn an_unresolved_tpd0_named_i2c0_is_swept_for_on_that_pci_function_tried_first() {
    let _core = core();
    // Gemini Lake's I2C1 (8086:31AE) is listed first, at id 0.
    let i2c1 = DeviceRecord { device: 0x31AE, ..lpss(0) };
    list(&table(&[i2c1, lpss(1)], &[hid(u64::MAX, I2C0, 0)]), None);
    let (cands, pads) = discovered();
    assert_eq!(order(&cands, &pads), [1, 0], "the function its name names first");
    let d = settled_on(&cands[1], HZ, &pads[0], Presence::Absent, &pads).expect("bind");
    assert_eq!((d.bound_addr, d.bound_by_probe), (0x15, true), "the sweep found the pad");
    assert_eq!(reached(&d.reach.get()), [0x15]);
}

#[test]
fn an_unresolved_pad_is_never_paired_with_a_platform_controller_by_its_name() {
    let _core = core();
    // Even a platform record whose device field reads as Gemini Lake's I2C0.
    let named_like = DeviceRecord { device: 0x31AC, ..platform(1) };
    list(&table(&[named_like], &[hid(u64::MAX, I2C0, 0), hid(u64::MAX, I2C0, 0x2C)]), None);
    let (cands, pads) = discovered();
    assert!(cands[0].is_acpi && pads.iter().all(|p| !p.names(&cands[0])));
    let d = settled_on(&cands[0], HZ, &pads[1], Presence::Absent, &pads).expect("bind");
    assert!(regs::with(|c| c.bus().trace().is_empty()), "swept by name on a platform host");
    assert_eq!((d.bound_addr, reached(&d.reach.get())), (0x2C, vec![0x2C]));
}

#[test]
fn an_unresolved_pad_with_no_i2cn_is_not_swept_on_an_unlisted_lpss_function() {
    let _core = core();
    // An Intel LPSS function the table does not list, taken by its class.
    let unlisted = DeviceRecord { device: 0xABCD, ..lpss(0) };
    list(&table(&[unlisted, platform(1)], &[hid(u64::MAX, 0, 0)]), None);
    let (cands, pads) = discovered();
    assert!(!pads[0].names(&cands[0]), "a pad with no name paired with an unlisted function");
    let d = settled_on(&cands[0], HZ, &pads[0], Presence::Absent, &pads).expect("bind");
    assert!(regs::with(|c| c.bus().trace().is_empty()), "swept on a function it does not name");
    assert_eq!(d.bound_addr, 0);
}

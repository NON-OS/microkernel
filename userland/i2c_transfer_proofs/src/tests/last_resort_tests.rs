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

//! The last resort: when no controller a pad names came up, setup keeps the
//! first that did and puts the declared pads in reach (setup/sequence/
//! run.rs, `declared` in order.rs). Only the pads that may sit on that
//! controller are put there: those naming it, by id or, unresolved, by
//! "I2Cn", and those naming no controller; never one whose record names
//! another. The shipping OP_ACPI_HID handler then names each in turn.

use nonos_i2cmodel::{Bus, Config, Designware};

use super::declared::reached;
use super::listed::{discovered, list, table};
use super::multi_sku::named;
use super::records::{hid, lpss, platform, I2C0};
use crate::bench::{driver, GEMINI_LAKE_HZ as HZ};
use crate::regs;
use crate::setup::order::declared;

#[test]
fn the_last_resort_reaches_only_the_pads_that_may_sit_on_its_controller() {
    let _core = regs::attach(Designware::new(Config::LPSS, Bus::default()));
    let (here, elsewhere) = (hid(0, 0, 0x2A), hid(1, 0, 0x2C));
    let (nowhere, by_name) = (hid(u64::MAX, 0, 0x10), hid(u64::MAX, I2C0, 0x15));
    list(&table(&[lpss(0), platform(1)], &[here, elsewhere, nowhere, by_name]), None);
    let (cands, pads) = discovered();
    let d = driver(HZ, 0).expect("bring-up");
    d.reach.set(declared(&cands[0], &pads));
    assert_eq!(reached(&d.reach.get()), [0x10, 0x15, 0x2A], "another's pad in reach");
    assert_eq!(named(&d, 4), [0x2A, 0x10, 0x15, 0x2A]);
    d.reach.set(declared(&cands[1], &pads));
    assert_eq!(reached(&d.reach.get()), [0x10, 0x2C], "the platform controller's own");
}

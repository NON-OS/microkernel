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

//! Where the sweep for a pad with no declared address never runs: with
//! nothing declared, with the declaration naming another controller or
//! none, and with only declarations that carry an address. Nothing is put
//! on the bus at all. And on a bus that faults it stops at the first try.

use nonos_i2cmodel::Config;

use super::declared::{here, nowhere, on, this};
use super::fixture::{bench, core};
use crate::constants::HID_INFO_TEN_BIT;
use crate::discover::AcpiTouchpad;
use crate::setup::settle::settle;
use crate::setup::sweep::sweep;
use crate::transaction::{BusState, Presence};

#[test]
fn with_nothing_declared_nothing_is_probed() {
    let b = bench(Config::LPSS);
    assert_eq!(sweep(&b.driver, &this(), &[]), None);
    assert!(core(|c| c.bus().trace().is_empty()), "the bus was probed");
}

#[test]
fn nothing_is_probed_without_an_unaddressed_declaration_on_this_controller() {
    let ten_bit = AcpiTouchpad { info: HID_INFO_TEN_BIT, ..here(0) };
    let cases: [(AcpiTouchpad, Vec<AcpiTouchpad>); 4] = [
        (on(3, 0), vec![on(3, 0)]),
        (nowhere(0), vec![nowhere(0)]),
        (ten_bit, vec![ten_bit]),
        (here(0x2C), vec![here(0x2C), on(3, 0)]),
    ];
    for (tg, targets) in cases {
        let b = bench(Config::LPSS);
        let s = settle(&b.driver, &this(), &tg, Presence::Absent, &targets);
        assert!(core(|c| c.bus().trace().is_empty()), "probed for {:#04x}", tg.addr);
        assert_eq!(s.addr, if tg.addressed() { tg.addr } else { 0 });
    }
}

#[test]
fn a_bus_that_faults_ends_the_sweep_at_its_first_address() {
    let b = bench(Config::LPSS);
    core(|c| c.stall_bus(false));
    assert_eq!(sweep(&b.driver, &this(), &[here(0)]), None);
    assert_eq!(b.driver.bus.get(), BusState::Faulting(1), "probed past a fault");
}

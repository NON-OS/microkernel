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

//! A run of faults ends in a reset of the controller with a named outcome
//! (plan/50 C1), not in a bus that stays dead until the next boot.

use nonos_i2cmodel::regs::LPSS_PRIV_RESETS;
use nonos_i2cmodel::{descriptor, Config};
use nonos_libc::{mk_uptime_ms, take_debug};

use super::fixture::{bench, core};
use super::pace_tests::{read, DESC};
use crate::transaction::{transfer, BusState, TransferRequest};

#[test]
fn a_controller_that_wedged_is_reset_and_the_next_transfer_goes_through() {
    let b = bench(Config::LPSS);
    core(|c| c.stall_bus(true));
    assert!(read(&b).is_err());
    assert_eq!(b.driver.bus.get(), BusState::Faulting(1));
    assert!(read(&b).is_err());
    assert_eq!(b.driver.bus.get(), BusState::Reset, "two faults in a row reset the core");
    assert_eq!(read(&b).ok().as_deref(), Some(&b.pad.lock().descriptor()[..4]));
    assert_eq!(b.driver.bus.get(), BusState::Ready);
    core(|c| {
        assert_eq!(c.writes_to(LPSS_PRIV_RESETS), [0, 3, 0, 3], "bring-up, then one recovery");
        assert!(c.violations().is_empty(), "{:?}", c.violations());
    });
    let full = TransferRequest { read_len: descriptor::LEN, ..DESC };
    assert!(transfer(&b.driver, full).is_ok(), "the bus works after the recovery");
    let lines = take_debug();
    assert!(lines.iter().any(|l| l.contains("controller reset")), "{lines:?}");
    assert!(lines.iter().any(|l| l.contains("moving again")), "{lines:?}");
}

#[test]
fn a_bus_no_reset_frees_is_named_stuck_and_still_answers_every_call() {
    let b = bench(Config::LPSS);
    core(|c| c.stall_bus(false));
    for _ in 0..6 {
        let t = mk_uptime_ms();
        assert!(read(&b).is_err());
        assert!(mk_uptime_ms() - t < 400, "one call held the bus too long");
    }
    assert_eq!(b.driver.bus.get(), BusState::Stuck(0), "reset again on the sixth fault");
    let resets = core(|c| c.writes_to(LPSS_PRIV_RESETS).len());
    assert_eq!(resets, 2 + 2 * 3, "a reset after every second fault, no more");
    let said = take_debug().iter().filter(|l| l.contains("still stuck")).count();
    assert_eq!(said, 1, "the stuck bus is said once, not per fault");
}

#[test]
fn a_nack_is_the_device_answering_and_ends_a_run_of_faults() {
    let b = bench(Config::LPSS);
    for was in [BusState::Faulting(1), BusState::Reset, BusState::Stuck(1)] {
        b.driver.bus.set(was);
        let nobody = TransferRequest { addr: 0x2C, ..DESC };
        assert!(transfer(&b.driver, nobody).is_err(), "nothing lives at 0x2C");
        assert_eq!(b.driver.bus.get(), BusState::Ready, "a NACK after {was:?} is not a fault");
    }
    assert_eq!(core(|c| c.writes_to(LPSS_PRIV_RESETS).len()), 2, "no reset for a NACK");
}

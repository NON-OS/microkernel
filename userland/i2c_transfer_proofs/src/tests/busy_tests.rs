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

//! A wait on a busy machine, where every yield can hand a whole time slice
//! to other work. While the bus moves, a wait polls back to back and gives
//! nothing up; once it does yield, it reads the clock after every yield, so
//! it ends within one slice of its deadline however many polls that is.

use nonos_i2cmodel::Config;
use nonos_libc::{busy_scheduler, mk_uptime_ms, yields};

use super::fixture::bench;
use super::pace_tests::read;
use crate::transaction::pace::Pace;

/// A scheduler tick on a loaded machine.
const SLICE_MS: u64 = 10;

#[test]
fn a_wait_under_a_busy_scheduler_ends_within_one_slice_of_its_deadline() {
    let _busy = busy_scheduler(SLICE_MS);
    for budget in [0, 1, 2, 5, 25, 50, 300] {
        let t = mk_uptime_ms() as u64;
        let mut pace = Pace::new(budget);
        while pace.wait() {}
        let waited = mk_uptime_ms() as u64 - t;
        // The shim's clock moves a millisecond per read: Pace::new, the read
        // that ends the wait, and the test's own.
        assert!(waited >= budget, "budget {budget} ms ended after {waited} ms");
        assert!(waited <= budget + SLICE_MS + 3, "budget {budget} ms ended after {waited} ms");
    }
}

#[test]
fn a_transfer_on_a_moving_bus_polls_back_to_back_and_never_yields() {
    let b = bench(Config::LPSS);
    let _busy = busy_scheduler(SLICE_MS);
    let (y, t) = (yields(), mk_uptime_ms());
    assert!(read(&b).is_ok());
    assert_eq!(yields(), y, "a healthy transfer gave the processor up");
    assert!(mk_uptime_ms() - t < SLICE_MS as i64, "it took a slice");
}

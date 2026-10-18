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

//! A teardown on a controller that will not enter reset: CRST is written
//! low and keeps reading high. The wait on it is bounded and its timeout
//! named, and the teardown goes on to the release regardless, last.

use std::time::{Duration, Instant};

use nonos_libc::logged;

use crate::constants::{GCTL, GCTL_CRST};
use crate::controller::reset::RESET_MS;
use crate::error::{reason, HdaError};
use crate::model::live;
use crate::proofs::lifecycle::teardown::running;
use crate::proofs::lifecycle::teardown_log::{release, teardown};

#[test]
fn a_controller_held_out_of_reset_is_waited_on_its_bound_named_and_still_let_go() {
    let named = reason(HdaError::ControllerResetTimeout);
    // The model is a thread; a run in which the driver read CRST low before
    // the model set it again proves nothing, so it is run again.
    for _ in 0..20 {
        let bar = running();
        let _held = live(&bar, |b| {
            b.present8(GCTL as usize, b.wrote8(GCTL as usize) | GCTL_CRST as u8);
            std::thread::yield_now();
        });
        let (seen, t) = (logged().len(), Instant::now());
        let log = teardown(&bar);
        let took = t.elapsed();
        if !logged()[seen..].iter().any(|l| l.contains(named)) {
            continue;
        }
        assert!(took >= Duration::from_millis(RESET_MS), "gave up on CRST {took:?} in");
        assert!(took.as_secs() < 2, "waited on CRST without a bound: {took:?}");
        release(&log);
        return;
    }
    panic!("in 20 runs the driver never named the reset it could not enter");
}

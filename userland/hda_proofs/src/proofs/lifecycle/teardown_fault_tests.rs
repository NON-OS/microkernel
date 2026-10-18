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

//! A teardown on a controller that does not cooperate: an engine that never
//! reads stopped, and a controller that has dropped off the bus. Neither may
//! hold the teardown, and neither may keep the claim from being given back.

use std::time::Instant;

use nonos_libc::{logged, MK_PCI_CMD_BUS_MASTER};

use crate::constants::GCTL_CRST;
use crate::error::{reason, HdaError};
use crate::proofs::lifecycle::teardown::{running, with_stuck_engine};
use crate::proofs::lifecycle::teardown_log::{release, teardown, CMD, GCTL_AT};

#[test]
fn an_engine_that_will_not_stop_is_named_and_the_controller_reset_regardless() {
    let named = reason(HdaError::EngineStopTimeout);
    let (log, took) = with_stuck_engine(|bar| {
        let (seen, t) = (logged().len(), Instant::now());
        let log = teardown(bar);
        let took = t.elapsed();
        logged()[seen..].iter().any(|l| l.contains(named)).then_some((log, took))
    });
    assert!(took.as_secs() < 2, "the teardown waited on the engine without a bound");
    let r = &release(&log).regs;
    assert_eq!(r[GCTL_AT] & GCTL_CRST, 0, "the stuck engine was not stopped by a reset");
    assert_eq!(r[CMD] as u16 & MK_PCI_CMD_BUS_MASTER, 0, "bus master still on at release");
}

#[test]
fn a_controller_that_reads_all_ones_is_not_waited_on_and_still_let_go() {
    let bar = running();
    (0..bar.len()).for_each(|i| bar.present8(i, 0xff));
    let log = teardown(&bar);
    release(&log);
    assert!((0..bar.len()).all(|i| bar.wrote8(i) == 0xff), "wrote to a dead window");
    let named = reason(HdaError::ControllerNotResponding);
    assert!(logged().iter().any(|l| l.contains(named)), "{:?}", logged());
}

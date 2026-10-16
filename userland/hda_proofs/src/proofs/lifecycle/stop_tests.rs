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

//! The engines a previous owner left running, stopped before the reset.
//!
//! The reset used to tell every engine to stop and then reset the controller
//! whether or not any had: the waits on the ring engines were discarded and
//! the streams were not waited on at all. A reset under a running bus master
//! is undefined. Now an engine that does not read stopped in time fails the
//! attempt by name, the controller is left out of reset, and the attempt's
//! teardown resets it for the next one.

use crate::constants::{CORBCTL, CORBCTL_RUN, GCTL, GCTL_CRST, RIRBCTL, RIRBCTL_DMAEN, SDCTL_RUN};
use crate::controller::reset::reset_link;
use crate::controller::stop::stop_engines;
use crate::error::HdaError;
use crate::proofs::lifecycle::teardown::{running, sd_ctl, with_stuck_engine};
use crate::regs::Regs;

#[test]
fn a_stream_that_will_not_stop_fails_the_reset_by_name_and_is_not_reset_under() {
    let (r, crst) = with_stuck_engine(|bar| {
        let r = reset_link(Regs::new(bar.base()));
        r.is_err().then(|| (r, bar.wrote32(GCTL as usize) & GCTL_CRST))
    });
    assert_eq!(r, Err(HdaError::EngineStopTimeout));
    assert_ne!(crst, 0, "CRST was taken low under the running engine");
}

#[test]
fn every_other_engine_is_stopped_even_when_one_will_not() {
    let left = with_stuck_engine(|bar| {
        bar.present8(sd_ctl(5), SDCTL_RUN);
        let r = stop_engines(Regs::new(bar.base()));
        let at = |off: usize, bit: u8| bar.wrote8(off) & bit;
        let left = [at(sd_ctl(5), SDCTL_RUN), at(CORBCTL as usize, CORBCTL_RUN)];
        (r == Err(HdaError::EngineStopTimeout)).then(|| (left, at(RIRBCTL as usize, RIRBCTL_DMAEN)))
    });
    assert_eq!(left, ([0, 0], 0), "(stream 5, CORB) or the RIRB was left running");
}

#[test]
fn engines_that_stop_let_the_reset_go_ahead() {
    let bar = running();
    assert_eq!(stop_engines(Regs::new(bar.base())), Ok(()));
    assert_eq!(bar.wrote8(sd_ctl(4)) & SDCTL_RUN, 0);
}

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

//! The order inside the teardown's quiesce, witnessed while it waits: the
//! streams, then the CORB and RIRB, then the position buffer, then CRST. A
//! register window cannot record the order of writes, so each test keeps
//! one part from moving and watches what the driver does meanwhile
//! (`witness`).

use std::time::Duration;

use nonos_devmodel::FakeBar;

use crate::constants::{CORBCTL, CORBCTL_RUN, DPLBASE, GCTL, GCTL_CRST, SDCTL_RUN};
use crate::proofs::lifecycle::teardown::sd_ctl;
use crate::proofs::lifecycle::witness::{first_seen, waited_out};

const CRST: u8 = GCTL_CRST as u8;

/// Set `bit` of register `off` again whenever the driver clears it, and
/// say whether it had.
fn hold(b: &FakeBar, off: usize, bit: u8) -> bool {
    let v = b.wrote8(off);
    if v & bit == 0 {
        b.present8(off, v | bit);
    }
    v & bit == 0
}

#[test]
fn the_rings_are_told_to_stop_only_once_every_stream_has_been_waited_on() {
    let (t, _) = waited_out(|b| {
        hold(b, sd_ctl(4), SDCTL_RUN);
        b.wrote8(CORBCTL as usize) & CORBCTL_RUN == 0
    });
    assert!(t >= Duration::from_millis(5), "CORB told to stop {t:?} in, under a running stream");
}

#[test]
fn the_position_buffer_goes_off_only_once_the_rings_have_been_waited_on() {
    let (t, _) = waited_out(|b| {
        hold(b, CORBCTL as usize, CORBCTL_RUN);
        b.wrote32(DPLBASE as usize) == 0
    });
    assert!(t >= Duration::from_millis(50), "DPLBASE cleared {t:?} in, under a running CORB");
}

#[test]
fn the_controller_is_put_in_reset_only_once_the_position_buffer_is_off() {
    let (_, dpl) = first_seen(|b| hold(b, GCTL as usize, CRST));
    assert_eq!(dpl, 0, "CRST taken low with the position buffer still on");
}

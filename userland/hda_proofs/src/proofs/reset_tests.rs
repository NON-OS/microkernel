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
//! A full controller reset, and the codecs that answer after it.
//!
//! The driver used to leave a controller whose CRST already read set alone
//! and take STATESTS as it found it. UEFI firmware does exactly that set-up
//! and then clears STATESTS behind it, so on a real laptop the driver saw a
//! running controller with no codecs and reported the machine silent. These
//! hold the reset to Linux's `snd_hdac_bus_reset_link`: engines stopped,
//! CRST taken low and seen low, released and seen high, and STATESTS read
//! after the codecs had their time to signal.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use crate::constants::{
    CORBCTL, CORBCTL_RUN, GCAP, GCTL, GCTL_CRST, INTCTL, SDCTL_IOCE, SDCTL_RUN, SD_CTL, STATESTS,
};
use crate::controller::reset::{leave_reset, reset_link};
use crate::model::{live, window};
use crate::regs::Regs;

/// GCTL's unsolicited response enable, a bit the reset must carry over.
const GCTL_UNSOL: u32 = 1 << 8;

/// A link whose codecs at `mask` raise their STATESTS bits each time CRST
/// goes from low to high, as the specification has them do.
fn link_with_codecs(mask: u16) -> impl Fn(&nonos_devmodel::FakeBar) + Send {
    let in_reset = Arc::new(AtomicBool::new(false));
    move |b| {
        let crst = b.wrote32(GCTL as usize) & GCTL_CRST != 0;
        if !crst {
            in_reset.store(true, Ordering::Release);
        } else if in_reset.swap(false, Ordering::AcqRel) {
            b.present16(STATESTS as usize, mask);
        }
        std::thread::yield_now();
    }
}

#[test]
fn a_controller_firmware_left_running_is_reset_and_its_codecs_found_again() {
    /*
     * The model raises STATESTS on the rising edge of CRST, so it has to be
     * scheduled during the millisecond the driver holds the controller in
     * reset. Under a loaded test run it sometimes is not, which proves
     * nothing either way, so the run is repeated until it sees the edge.
     */
    let found = (0..20).any(|_| {
        std::thread::sleep(std::time::Duration::from_millis(10));
        let bar = window();
        bar.present32(GCTL as usize, GCTL_CRST);
        bar.present16(STATESTS as usize, 0);
        let _link = live(&bar, link_with_codecs(0b101));
        reset_link(Regs::new(bar.base())) == Ok(0b101)
    });
    assert!(found, "the codecs at 0 and 2 that signalled after the reset were missed");
}

#[test]
fn the_reset_keeps_the_rest_of_the_control_register() {
    let bar = window();
    bar.present32(GCTL as usize, GCTL_UNSOL);
    assert!(leave_reset(Regs::new(bar.base())).is_ok());
    assert_eq!(bar.wrote32(GCTL as usize), GCTL_UNSOL | GCTL_CRST);
}

#[test]
fn engines_a_previous_owner_left_running_are_stopped_before_the_reset() {
    let bar = window();
    bar.present16(GCAP as usize, 0x4401);
    bar.present8(CORBCTL as usize, CORBCTL_RUN);
    let first_out = 0x80 + 4 * 0x20;
    bar.present8(first_out + SD_CTL as usize, SDCTL_RUN | SDCTL_IOCE);
    bar.present32(INTCTL as usize, 0xc000_0010);
    let _link = live(&bar, link_with_codecs(1));
    assert!(reset_link(Regs::new(bar.base())).is_ok());
    assert_eq!(bar.wrote8(first_out + SD_CTL as usize) & SDCTL_RUN, 0, "a stream kept running");
    assert_eq!(bar.wrote8(CORBCTL as usize) & CORBCTL_RUN, 0, "the CORB kept running");
    assert_eq!(bar.wrote32(INTCTL as usize), 0, "interrupts stayed enabled across the reset");
}

#[test]
fn an_empty_link_is_reported_empty_within_a_bounded_wait() {
    let bar = window();
    let t = Instant::now();
    assert_eq!(reset_link(Regs::new(bar.base())), Ok(0));
    assert!(t.elapsed().as_millis() < 1000, "waiting for absent codecs held boot");
}

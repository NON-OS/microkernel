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

//! A codec that answers nothing while jack events keep arriving. Each event
//! reads the jack at once, back-off or not, so the codec is lost within a
//! second; after that no event reads it again, and the line saying it was
//! lost is logged once however many passes the serving loop makes.

use std::time::{Duration, Instant};

use nonos_libc::logged;

use super::jack_poll::JackPoll;
use super::jack_schedule::MAX_FAILS;
use super::serving::{playing, Playing};
use crate::constants::{RIRBWP, RIRB_EX_UNSOL};

const LOST: &str = "headphone jack no longer read";

/// One pass of the serving loop with a jack event before it: the controller
/// writes the next RIRB entry, marked unsolicited, and moves RIRBWP onto it.
fn pass(jack: &mut JackPoll, p: &mut Playing) {
    let (bar, rirb) = (&p.b.sim.bar, &p.b.sim.rirb);
    let wp = (bar.wrote16(RIRBWP as usize) + 1) & 0xff;
    rirb.present32(wp as usize * 8 + 4, RIRB_EX_UNSOL);
    bar.present16(RIRBWP as usize, wp);
    jack.poll(&mut p.d);
    std::thread::sleep(Duration::from_millis(5));
}

#[test]
fn a_codec_lost_under_a_stream_of_jack_events_is_logged_once_and_not_read_again() {
    let mut p = playing();
    // Nothing answers at address 5: every verb to it goes unanswered.
    p.d.codec.cad = 5;
    let (from, asked) = (logged().len(), p.b.sim.sent().len());
    let lost = || logged()[from..].iter().filter(|l| l.contains(LOST)).count();
    let (mut jack, end) = (JackPoll::new(), Instant::now() + Duration::from_secs(20));
    while lost() == 0 && Instant::now() < end {
        pass(&mut jack, &mut p);
    }
    for _ in 0..200 {
        pass(&mut jack, &mut p);
    }
    assert_eq!(lost(), 1, "lines logged saying the codec was lost");
    let reads = p.b.sim.sent().len() - asked;
    assert_eq!(reads, MAX_FAILS as usize, "the jack was read {reads} times");
}

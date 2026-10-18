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

//! The jack poll in the serving loop, on its own clock: it follows the
//! headphones while the codec answers, and backs off from one that stopped,
//! so a dead codec is not asked again inside its back-off.

use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use nonos_libc::logged;

use super::jack_poll::JackPoll;
use super::serving::{playing, Playing};

/// Poll the jack every 5 ms for `ms`, or until `done`.
fn poll_for(p: &mut Playing, ms: u64, done: fn(&Playing) -> bool) {
    let (mut jack, end) = (JackPoll::new(), Instant::now() + Duration::from_millis(ms));
    while Instant::now() < end && !done(p) {
        jack.poll(&mut p.d);
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn headphones_plugged_in_switch_the_speakers_off_at_the_next_read() {
    let mut p = playing();
    p.b.sim.plugged.store(true, Ordering::Release);
    let from = logged().len();
    poll_for(&mut p, 3_000, |p| p.d.status.plugged);
    assert!(p.d.status.plugged, "the plugged jack was not followed");
    let said = logged()[from..].iter().any(|l| l.starts_with("[HDA] headphones in"));
    assert!(said, "{:?}", &logged()[from..]);
}

#[test]
fn a_codec_that_stops_answering_is_asked_once_then_left_alone_while_backing_off() {
    let mut p = playing();
    // Nothing answers at address 5: every verb to it goes unanswered.
    p.d.codec.cad = 5;
    let from = p.b.sim.sent().len();
    poll_for(&mut p, 1_400, |_| false);
    let asked: Vec<u32> = p.b.sim.sent()[from..].to_vec();
    assert_eq!(asked.len(), 1, "asked {asked:x?} inside the back-off");
    assert_eq!(asked[0] >> 28, 5);
}

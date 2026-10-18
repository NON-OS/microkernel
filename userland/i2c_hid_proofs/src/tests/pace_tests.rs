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

//! Without an interrupt line the pad is read on a timer, and the timer
//! backs off while the pad is quiet: every 2 ms while reports come, no more
//! than every 16 ms once a second has passed without one.

use nonos_i2cmodel::touchpad::{touch_report, Touch};
use nonos_i2cmodel::Command;

use super::driven::bound;
use super::fixture::Rig;
use crate::drive::pass;
use crate::state::State;

/// The loop's passes over `ms` of its time from `start`, one a millisecond;
/// the input reads the pad saw. `touch` queues a finger before each pass.
fn run(r: &Rig, s: &mut State, start: u64, ms: u64, touch: bool) -> usize {
    r.pad.lock().take_commands();
    for t in start..start + ms {
        if touch && t % 2 == 0 {
            r.pad.lock().push_input(&touch_report(&Touch::finger(100 + (t % 50) as u16, 100)));
        }
        pass(s, t);
    }
    let reads = |c: &&Command| matches!(c, Command::InputRead | Command::RegisterRead(3));
    r.pad.lock().commands().iter().filter(reads).count()
}

#[test]
fn an_idle_pad_is_read_no_more_than_every_16_ms() {
    let (r, mut s) = bound();
    run(&r, &mut s, 10_000, 2_000, false);
    let reads = run(&r, &mut s, 12_000, 4_000, false);
    assert!(reads <= 4_000 / 16 + 1, "{reads} reads in four idle seconds");
    assert!(reads >= 4_000 / 17, "an idle pad was not read at all: {reads}");
}

#[test]
fn a_pad_with_a_finger_on_it_is_read_every_2_ms_and_backs_off_after_a_second() {
    let (r, mut s) = bound();
    let busy = run(&r, &mut s, 10_000, 1_000, true);
    assert!(busy >= 1_000 / 2 - 1, "reads while reports came: {busy}");
    let grace = run(&r, &mut s, 11_000, 1_000, false);
    assert!(grace >= 1_000 / 2 - 1, "the pad backed off within a second: {grace}");
    let quiet = run(&r, &mut s, 12_000, 2_000, false);
    assert!(quiet <= 2_000 / 16 + 1, "{quiet} reads in two quiet seconds");
}

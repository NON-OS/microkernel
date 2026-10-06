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

//! `install` waits on the installer only briefly and follows a load still
//! running from the job's ticks: the new child starts the drain, an
//! installer that answers a probe with no child fails it, the deadline ends
//! it, and probes back off so the terminal holds fewer than the kernel's
//! eight places in the installer's reply queue.

use crate::install_follow::{
    Follow, Step, DEADLINE_MS, FIRST_PROBE_MS, LOOK_GAP_MS, PROBE_GAP_MAX_MS,
};

fn fresh() -> Follow {
    Follow::new(vec![7, 9], 10_000)
}

#[test]
fn it_waits_its_first_probe_gap() {
    let f = fresh();
    assert_eq!(f.step(10_000, None), Step::Wait);
    assert_eq!(f.step(10_000 + FIRST_PROBE_MS, None), Step::Probe);
}

#[test]
fn a_new_child_is_the_loaded_program() {
    let mut f = fresh();
    assert_eq!(f.step(10_050, Some(31)), Step::Loaded(31));
    f.probed(10_600, true);
    assert_eq!(f.step(10_600, Some(31)), Step::Loaded(31));
}

#[test]
fn a_finished_installer_with_no_child_failed_the_load() {
    let mut f = fresh();
    f.probed(10_500, true);
    assert_eq!(f.step(10_500, None), Step::Failed);
}

#[test]
fn the_deadline_ends_a_load_that_never_settles() {
    let f = fresh();
    assert_eq!(f.step(10_000 + DEADLINE_MS, None), Step::TimedOut);
}

#[test]
fn probes_back_off_within_the_reply_places() {
    let mut f = fresh();
    let mut probes = 0;
    let mut now = 10_000;
    let mut gaps = Vec::new();
    let mut last = now;
    while now < 10_000 + DEADLINE_MS {
        if f.step(now, None) == Step::Probe {
            probes += 1;
            gaps.push(now - last);
            last = now;
            f.probed(now, false);
        }
        now += 10;
    }
    assert!(probes < 8, "{probes} probes");
    assert!(gaps.windows(2).all(|w| w[1] >= w[0] || w[1] >= PROBE_GAP_MAX_MS), "{gaps:?}");
}

#[test]
fn the_table_is_read_at_most_every_look_gap() {
    let mut f = fresh();
    assert!(f.due(10_000));
    assert!(!f.due(10_000 + LOOK_GAP_MS - 1));
    assert!(f.due(10_000 + LOOK_GAP_MS));
    assert_eq!(f.before, vec![7, 9]);
    assert_eq!(f.elapsed_ms(10_400), 400);
}

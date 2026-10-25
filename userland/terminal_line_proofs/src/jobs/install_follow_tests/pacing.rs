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

//! How often a followed load is probed, and the backoff that spares the reply queue.

use super::fresh;
use crate::install_follow::{Step, DEADLINE_MS, FIRST_PROBE_MS, LOOK_GAP_MS, PROBE_GAP_MAX_MS};

#[test]
fn it_waits_its_first_probe_gap() {
    let f = fresh();
    assert_eq!(f.step(10_000, None), Step::Wait);
    assert_eq!(f.step(10_000 + FIRST_PROBE_MS, None), Step::Probe);
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
            f.probed(now, None);
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

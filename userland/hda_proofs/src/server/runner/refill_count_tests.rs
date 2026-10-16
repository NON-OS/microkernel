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

//! What the refill counts and says: one line for the first period played,
//! one for the first underrun of a dry spell however long it lasts, and the
//! totals once the stream stops.

use nonos_libc::logged;

use super::refill::{refill, Refill};
use super::serving::{at, playing};
use crate::audio::PcmQueue;
use crate::controller::bdl::PERIOD_BYTES;

const PERIOD: usize = PERIOD_BYTES as usize;

#[test]
fn a_queue_that_runs_dry_is_one_underrun_however_long_it_stays_dry() {
    let p = playing();
    let (mut q, mut rf, from) = (PcmQueue::new(), Refill::new(true), logged().len());
    q.push(&vec![1; PERIOD]);
    for k in 1..=12u32 {
        at(&p, (k % 4) * PERIOD as u32, 0);
        refill(&p.d, &mut q, &mut rf);
    }
    rf.report();
    let lines = logged()[from..].to_vec();
    let count = |s: &str| lines.iter().filter(|l| l.starts_with(s)).count();
    assert_eq!((count("[HDA] refill"), count("[HDA] underrun")), (1, 1), "{lines:?}");
    assert_eq!(count("[HDA] stream stop: 1 periods played, 1 underruns"), 1, "{lines:?}");
}

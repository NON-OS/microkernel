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

//! Stopping: the outputs muted before the engine is stopped, the queue
//! emptied, the run's totals said once; and a start the box refuses.

use super::feed::period;
use super::util::{ask, request, status, up};
use crate::constants::ring::{PERIOD_BYTES, RING_BYTES};
use crate::model::{MASTER, NABM_GRANT, NAM_GRANT};
use crate::server::Stream;

#[test]
fn stop_mutes_then_halts_and_empties_the_queue() {
    let (mut d, ich) = up();
    let mut st = Stream::default();
    assert!(st.start(&mut d));
    let mut fed = 0;
    for _ in 0..6 {
        period(&mut d, &mut st, &ich, &mut fed, 12 * PERIOD_BYTES);
    }
    let mark = ich.borrow().writes.len();
    assert_eq!(status(&ask(&mut d, &mut st, &request(9, 3, &[]))), 0);
    let ich = ich.borrow();
    let after = &ich.writes[mark..];
    let mute = after.iter().position(|w| *w == (NAM_GRANT, (MASTER * 2) as u16, 0x8000));
    let halt = after.iter().position(|w| *w == (NABM_GRANT, 0x1B, 0));
    assert!(mute.unwrap() < halt.unwrap(), "muted before the engine stops");
    assert_eq!((ich.cr & 1, ich.sr & 1, st.running), (0, 1, false));
    assert!(!ich.open());
    assert_eq!(st.q.pop_into(&mut [0u8; 16]), 0, "the queue is empty");
    let totals = nonos_libc::logged().iter().filter(|l| l.contains("stream stop")).count();
    assert_eq!(totals, 1);
}

#[test]
fn a_second_start_begins_on_silence_again() {
    let (mut d, ich) = up();
    let mut st = Stream::default();
    assert!(st.start(&mut d));
    let mut fed = 0;
    for _ in 0..8 {
        period(&mut d, &mut st, &ich, &mut fed, 8 * PERIOD_BYTES);
    }
    st.stop(&mut d);
    assert!(st.start(&mut d));
    let ring = nonos_libc::dma_at(d.ring_dev() as u64);
    // SAFETY: the ring is inside the grant `dma_at` found.
    let ring = unsafe { std::slice::from_raw_parts(ring, RING_BYTES) };
    assert!(ring.iter().all(|&b| b == 0) && st.wpos == 0);
}

#[test]
fn a_box_that_will_not_reset_refuses_the_start_and_stays_muted() {
    let (mut d, ich) = up();
    ich.borrow_mut().opt.rr_sticks = true;
    let mut st = Stream::default();
    assert_eq!(status(&ask(&mut d, &mut st, &request(8, 4, &[]))), -22);
    assert!(!st.running && !ich.borrow().open());
}

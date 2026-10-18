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

//! The period refill, against whatever position the controller reports.
//! LPIB and the position buffer entry are the controller's word, and the
//! refill copies a period's worth of queued PCM wherever they say the
//! engine has been, so no reading of either may put a write off the ring.

use super::refill::{refill, Refill};
use super::serving::{at, playing};
use crate::audio::PcmQueue;
use crate::controller::bdl::{PERIOD_BYTES, RING_BYTES};
use crate::proofs::lifecycle::board::GUARD;

const PERIOD: usize = PERIOD_BYTES as usize;
const RING: u32 = RING_BYTES as u32;
const HOSTILE: [u32; 7] = [0, 1, RING - 1, RING, RING + 1, 0x7fff_ffff, u32::MAX];

#[test]
fn queued_pcm_lands_in_each_period_the_engine_has_finished_and_no_further() {
    let p = playing();
    let (mut q, mut rf) = (PcmQueue::new(), Refill::new(p.d.posbuf_va.is_some()));
    (1..=3u8).for_each(|k| assert_eq!(q.push(&vec![k; PERIOD]), PERIOD));
    at(&p, 3 * PERIOD as u32, 0);
    refill(&p.d, &mut q, &mut rf);
    for k in 0..3 {
        let (first, last) = (p.b.pcm.wrote8(k * PERIOD), p.b.pcm.wrote8((k + 1) * PERIOD - 1));
        assert_eq!((first, last), (k as u8 + 1, k as u8 + 1), "period {k} was not refilled");
    }
    assert_eq!(p.b.pcm.wrote8(3 * PERIOD), 0, "the period the engine is in was written");
}

#[test]
fn no_position_either_source_reports_moves_a_write_off_the_ring() {
    let p = playing();
    let (mut q, mut rf) = (PcmQueue::new(), Refill::new(true));
    for &buffer in &HOSTILE {
        for &lpib in &HOSTILE {
            q.push(&[0xa5; 4096]);
            at(&p, lpib, buffer);
            refill(&p.d, &mut q, &mut rf);
        }
    }
    let past: Vec<u8> = (0..GUARD).map(|i| p.b.pcm.wrote8(RING as usize + i)).collect();
    assert!(past.iter().all(|&b| b == 0), "written past the ring: {past:?}");
}

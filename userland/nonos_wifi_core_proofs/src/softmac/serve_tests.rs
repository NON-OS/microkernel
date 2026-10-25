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

//! The station serves whatever state it is in: the status op reports the
//! bring-up stage and the counters in the layout the panel reads, a radio
//! that never came up answers every link request with "down", and a frame no
//! request family takes is refused rather than left unanswered.

use crate::soft::env::SimEnv;
use crate::soft::join::joined;
use crate::soft::rig::SimAir;
use crate::soft::wire::{control, netif};
use nonos_wifi_core::netif::MAX_RESPONSE;
use nonos_wifi_core::softmac::engine::STATUS_REFUSED;
use nonos_wifi_core::softmac::SoftStation;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn the_status_op_reports_stage_and_counters() {
    let mut st = joined();
    let s = control(&mut st, 4, &[]);
    assert_eq!((s.len(), s[0]), (61, 0));
    assert_eq!(s[53..57], [0; 4], "no roam has failed");
    assert_eq!(s[57..61], [0; 4], "no beacon refused");
    let rx_ring = u32::from_le_bytes(s[9..13].try_into().unwrap());
    assert_eq!(rx_ring, 0, "the join's frames are the join's, not the link's");
}

#[test]
fn a_radio_that_never_came_up_still_serves() {
    let env = SimEnv { clock: Rc::new(Cell::new(0)), draws: 0 };
    let mut st: SoftStation<SimAir, SimEnv> = SoftStation::new(None, env, 4);
    assert_eq!(netif(&mut st, 2, &[]).1, vec![0], "link down");
    assert_eq!(control(&mut st, 4, &[])[0], 4, "the stage it stopped at");
    assert_eq!(i32::from_le_bytes(control(&mut st, 1, &[1, b'x', 0])[..4].try_into().unwrap()), -1);
    st.tick();
}

#[test]
fn a_frame_neither_family_takes_is_refused() {
    let mut st = joined();
    let mut out = vec![0u8; MAX_RESPONSE];
    let n = st.handle(&[0x55; 24], &mut out);
    assert_eq!(i32::from_le_bytes(out[n - 4..n].try_into().unwrap()), STATUS_REFUSED);
    assert_eq!(st.handle(&[1, 2], &mut out), 24, "a short frame still gets the refusal");
}

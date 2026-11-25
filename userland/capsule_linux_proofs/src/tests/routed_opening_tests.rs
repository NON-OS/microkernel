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

//! Streams opening on the Nym mixnet each cost the serve loop a slice when
//! they step. However many are opening, a pass of the loop steps one at
//! most, sharing the pass's one wait with the quiet streams, and each gets
//! its turn within as many passes as there are streams.

#[path = "../../../capsule_linux/src/linux/net/routed/pass.rs"]
mod pass;
#[path = "../../../capsule_linux/src/linux/net/routed/opening_turn.rs"]
mod opening_turn;

use opening_turn::{may_step, turn};
use std::sync::Mutex;

/// pass.rs keeps one flag for the capsule's one serve loop; the tests here
/// take turns with it, as the loop's passes do.
static LOOP: Mutex<()> = Mutex::new(());

const STEP: &str = include_str!("../../../capsule_linux/src/linux/net/routed/step.rs");
const DRIVE: &str = include_str!("../../../capsule_linux/src/linux/net/routed/drive.rs");

/// One pass of the serve loop over `stepped` (each opening stream's last
/// step), every stream looked at once, as parked connects are each tick.
/// The streams that stepped.
fn pass(stepped: &mut [i64], now: i64) -> Vec<u32> {
    pass::begin();
    let mut out = Vec::new();
    for me in 0..stepped.len() as u32 {
        let next = turn(stepped.iter().enumerate().map(|(h, s)| (h as u32, *s)));
        if may_step(me, next, pass::take) {
            stepped[me as usize] = now;
            out.push(me);
        }
    }
    out
}

#[test]
fn n_opening_streams_take_one_slice_a_pass_at_most() {
    let _loop = LOOP.lock().unwrap_or_else(|e| e.into_inner());
    for n in [1usize, 2, 4, 7] {
        let mut stepped = vec![0i64; n];
        for now in 1..=50 {
            assert!(pass(&mut stepped, now).len() <= 1, "{n} streams, pass {now}");
        }
    }
}

#[test]
fn each_opening_stream_steps_within_n_passes() {
    let _loop = LOOP.lock().unwrap_or_else(|e| e.into_inner());
    for n in [2usize, 4, 7] {
        let mut stepped = vec![0i64; n];
        let mut last = vec![0i64; n];
        for now in 1..=(10 * n as i64) {
            for h in pass(&mut stepped, now) {
                let gap = now - last[h as usize];
                assert!(last[h as usize] == 0 || gap <= n as i64, "stream {h} waited {gap} passes");
                last[h as usize] = now;
            }
        }
        assert!(last.iter().all(|&l| l > 0), "every stream stepped");
    }
}

#[test]
fn a_pass_whose_wait_is_spent_steps_no_opening_stream() {
    let _loop = LOOP.lock().unwrap_or_else(|e| e.into_inner());
    let mut stepped = vec![0i64; 3];
    pass::begin();
    assert!(pass::take(), "a quiet stream takes the pass's wait first");
    let next = turn(stepped.iter().enumerate().map(|(h, s)| (h as u32, *s)));
    assert!(!may_step(next.unwrap(), next, pass::take));
    pass::begin();
    pass::spend();
    assert!(!may_step(next.unwrap(), next, pass::take), "input delivered spends it too");
    stepped[0] = 1;
}

#[test]
fn the_step_asks_its_turn_before_it_waits_on_net_socks5() {
    let at = |needle: &str| STEP.find(needle).unwrap_or_else(|| panic!("{needle} is gone"));
    let asked = at("if !super::opening_turn::may_step(turn.0, turn.1, super::pass::take) {");
    assert!(asked < at("Phase::Nym { opening, .. } => match opening.step(SLICE_MS) {"));
    assert!(DRIVE.contains("table::opening_turn()"));
}

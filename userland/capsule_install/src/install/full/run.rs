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

/*
 * The full-screen loop: a key goes to the window's key router, a tick moves
 * the write on by one budget, and the frame is drawn when either changed
 * something. While a disk is written a key waits at most a millisecond and
 * the screen is drawn every BUSY_FRAME_MS, so painting never holds it up.
 * While the disk list waits for a disk a key waits a quarter second, so
 * the list is looked at again on time with no key pressed.
 */

use alloc::vec;
use nonos_app_skeleton::clients::compositor::scene_remove;
use nonos_app_skeleton::EventOutcome;
use nonos_libc::{mk_exit, mk_ipc_recv_from, mk_time_millis};

use super::draw::{busy, draw, next};
use super::grab::{grab_keyboard, subscribe_keys};
use super::input::{parse, DELIVERY_LEN};
use super::start::start;
use crate::install::event::on_event;
use crate::install::job::tick;
use crate::install::state::State;
use crate::install::survey::{poll, watching};

const GRAB_RETRY_MS: u64 = 100;
const BUSY_WAIT_MS: u64 = 1;
const BUSY_FRAME_MS: i64 = 50;
/* A receive with no deadline: nothing moves until a key does. */
const BLOCK: u64 = 0;
/* While the disk list waits for a disk, often enough to look on time. */
const WATCH_WAIT_MS: u64 = 250;

pub fn run() -> ! {
    let s = start();
    let mut state = State::new();
    let (mut rid, mut held, mut drawn) = (4u32, false, 0i64);
    let _ = subscribe_keys(s.router, next(&mut rid));
    draw(&mut state, &s, &mut rid);
    let mut rx = vec![0u8; DELIVERY_LEN.max(64)];
    loop {
        if !held {
            held = grab_keyboard(s.router, next(&mut rid));
        }
        let wait = match (busy(&state), held) {
            (true, _) => BUSY_WAIT_MS,
            (false, false) => GRAB_RETRY_MS,
            (false, true) if watching(&state) => WATCH_WAIT_MS,
            (false, true) => BLOCK,
        };
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), wait, &mut sender);
        let key = if n > 0 { parse(&rx[..n as usize]) } else { None };
        let outcome = key.map_or(EventOutcome::Idle, |e| on_event(&mut state, e));
        if outcome == EventOutcome::Close {
            let _ = scene_remove(s.compositor, next(&mut rid), 0);
            mk_exit(0);
        }
        let stepped = tick(&mut state) | poll(&mut state);
        let now = mk_time_millis();
        let due = !busy(&state) || now.wrapping_sub(drawn) >= BUSY_FRAME_MS;
        if outcome == EventOutcome::Repaint || (stepped && due) {
            draw(&mut state, &s, &mut rid);
            drawn = now;
        }
    }
}

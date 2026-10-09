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

use core::sync::atomic::Ordering;

use super::super::ring_math;
use super::super::types::{InputEvent, RegistryError, INPUT_RING_CAP};
use super::ring::{Ring, DIAG, DROPPED, FIRST_INPUT_POST, KIND_DIAG_BASE, RING, SEQ, WAITER};

pub fn post_input(ev: InputEvent) -> Result<(), RegistryError> {
    if ev.kind >= KIND_DIAG_BASE {
        let slot = (ev.kind - KIND_DIAG_BASE) as usize;
        if let Some(c) = DIAG.get(slot) {
            c.fetch_add(1, Ordering::Relaxed);
        }
        return Ok(());
    }
    push(&mut RING.lock(), ev)?;
    published();
    Ok(())
}

/// `post_input` for interrupt context: `None`, with nothing posted, when the
/// ring's lock is held. The lock is a spin lock that syscall paths take, so
/// a timer tick that waited on it could spin on the CPU whose interrupted
/// code holds it. The caller keeps the event and tries again next tick.
pub fn try_post_input(ev: InputEvent) -> Option<Result<(), RegistryError>> {
    let mut ring = RING.try_lock()?;
    let r = push(&mut ring, ev);
    drop(ring);
    if r.is_ok() {
        published();
    }
    Some(r)
}

fn push(ring: &mut Ring, ev: InputEvent) -> Result<(), RegistryError> {
    if ring_math::is_full(ring.head, ring.tail, INPUT_RING_CAP) {
        DROPPED.fetch_add(1, Ordering::Relaxed);
        return Err(RegistryError::OutOfSlots);
    }
    let head = ring.head;
    ring.buf[head] = ev;
    ring.head = ring_math::wrap(head, INPUT_RING_CAP);
    Ok(())
}

fn published() {
    SEQ.fetch_add(1, Ordering::Release);
    crate::sys::bench::mark_once(&FIRST_INPUT_POST, b"input_post_first");
    let waiter = WAITER.swap(0, Ordering::AcqRel);
    if waiter != 0 {
        crate::sched::wake_process(waiter as u32);
    }
}

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

use super::super::types::{InputEvent, RegistryError};
use super::ring::{DIAG, KIND_DIAG_BASE, RING};
use super::ring_push::{published, push};
use super::source::{stamp, KERNEL};

/// The kernel's own post: source 0 (source.rs).
pub fn post_input(ev: InputEvent) -> Result<(), RegistryError> {
    post_stamped(ev, KERNEL)
}

/// MkInputEventPost by `pid`: stamped with its process's source, whatever
/// the poster wrote in the high byte of `flags`.
pub fn post_input_from(pid: u32, ev: InputEvent) -> Result<(), RegistryError> {
    post_stamped(ev, super::sources::source_of(pid))
}

fn post_stamped(mut ev: InputEvent, source: u8) -> Result<(), RegistryError> {
    ev.flags = stamp(ev.flags, source);
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
pub fn try_post_input(mut ev: InputEvent) -> Option<Result<(), RegistryError>> {
    ev.flags = stamp(ev.flags, KERNEL);
    let mut ring = RING.try_lock()?;
    let r = push(&mut ring, ev);
    drop(ring);
    if r.is_ok() {
        published();
    }
    Some(r)
}

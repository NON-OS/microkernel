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

//! An epoll descriptor watched in its turn, by poll, select or another
//! epoll: readable while an entry of it would report something, as Linux's
//! eventpoll_poll has it, and never writable. Looking does not use up an
//! edge-triggered entry's rise; only epoll_wait on the list itself does.

use alloc::vec::Vec;

use crate::linux::guest::{Guest, Kind};
use crate::linux::net::ready;

use super::epoll_rules::{live, MAX_NESTS};

const POLLIN: u16 = 0x001;
const POLLNVAL: u16 = 0x020;

/// The poll bits of the epoll `ep`.
pub fn bits(guest: &Guest, ep: u64) -> u16 {
    let mut seen = Vec::new();
    level(guest, ep, 0, &mut seen)
}

/*
 * Each epoll is looked at once per call, whatever watches it how often, so
 * no arrangement of lists makes one look long; one seen again while it is
 * being looked at, a loop epoll_ctl refuses to make, reads as not ready.
 */
fn level(guest: &Guest, ep: u64, depth: u32, seen: &mut Vec<(u64, u16)>) -> u16 {
    if let Some(&(_, b)) = seen.iter().find(|(fd, _)| *fd == ep) {
        return b;
    }
    let Some(list) = guest.fds.get(ep as usize).filter(|f| f.kind == Kind::Epoll) else {
        return POLLNVAL;
    };
    if depth > MAX_NESTS {
        return 0;
    }
    seen.push((ep, 0));
    let mut any = false;
    for w in list.watch.iter().filter(|w| w.armed) {
        let inner = guest.fds.get(w.fd as usize).is_some_and(|f| f.kind == Kind::Epoll);
        let now = match inner {
            true => level(guest, w.fd, depth + 1, seen),
            false => ready(guest, w.fd),
        };
        if live(u32::from(now), w.events, w.fired) != 0 {
            any = true;
            break;
        }
    }
    let b = if any { POLLIN } else { 0 };
    if let Some(entry) = seen.iter_mut().find(|(fd, _)| *fd == ep) {
        entry.1 = b;
    }
    b
}

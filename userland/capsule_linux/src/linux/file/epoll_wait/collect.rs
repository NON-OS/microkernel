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

//! What an epoll list reports now, entry by entry, as the rules in
//! epoll_rules.rs say, and what each entry has then seen.

use alloc::vec::Vec;

use crate::linux::guest::{Guest, Watch, EPOLLONESHOT};
use crate::linux::net::ready;

use super::super::epoll_rules::{live, seen};

/// The events of `watch` ready now, at most `max`, as the bytes epoll_wait
/// writes, and how many; each entry left as it is once they are reported.
pub(super) fn collect(guest: &Guest, watch: &mut [Watch], max: u64) -> (Vec<u8>, u64) {
    let mut blob: Vec<u8> = Vec::new();
    let mut hits = 0u64;
    for w in watch.iter_mut().filter(|w| w.armed) {
        if hits >= max {
            break;
        }
        // Hang-up and error are reported whether they were asked for or not.
        let level = u32::from(ready(guest, w.fd));
        let live = live(level, w.events, w.fired);
        w.fired = seen(level, w.events);
        if live == 0 {
            continue;
        }
        w.armed = w.events & EPOLLONESHOT == 0;
        blob.extend_from_slice(&live.to_le_bytes());
        blob.extend_from_slice(&w.data.to_le_bytes());
        hits += 1;
    }
    (blob, hits)
}

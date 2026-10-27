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

//! Where the streams live: one table for every guest, as the socket table
//! is, an entry named by its index from the socket's `Backend::Routed`.

use alloc::vec::Vec;
use core::cell::RefCell;

use nonos_route_link::STREAMS_PER_CALLER;

use super::entry::Entry;

struct One(RefCell<Vec<Option<Entry>>>);

/*
 * SAFETY: the serve loop is the only thread in this capsule that reaches the
 * table, as for the socket table (sock/cell.rs).
 */
unsafe impl Sync for One {}

static TABLE: One = One(RefCell::new(Vec::new()));

/// Run `f` on entry `h`, if it is there. Nothing inside `f` reaches this
/// table again or the socket table: a call to a proxy made inside reaches
/// neither.
pub fn with<R>(h: u32, f: impl FnOnce(&mut Entry) -> R) -> Option<R> {
    TABLE.0.borrow_mut().get_mut(h as usize).and_then(Option::as_mut).map(f)
}

pub fn insert(e: Entry) -> u32 {
    let mut t = TABLE.0.borrow_mut();
    match t.iter().position(Option::is_none) {
        Some(i) => {
            t[i] = Some(e);
            i as u32
        }
        None => {
            t.push(Some(e));
            (t.len() - 1) as u32
        }
    }
}

/// Entry `h`, out of the table. Dropping it ends its stream at the proxy,
/// which the caller does once the table is no longer borrowed.
pub fn remove(h: u32) -> Option<Entry> {
    TABLE.0.borrow_mut().get_mut(h as usize).and_then(Option::take)
}

/// The lowest stream at net.socks5 no entry holds: 1 to 7, stream 0 being
/// the one a frame naming none belongs to.
pub fn free_stream() -> Option<u32> {
    let t = TABLE.0.borrow();
    let held: Vec<u32> = t.iter().flatten().filter_map(Entry::stream).collect();
    (1..STREAMS_PER_CALLER).find(|s| !held.contains(s))
}

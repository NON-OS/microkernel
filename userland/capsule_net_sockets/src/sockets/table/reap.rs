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
 * A client that ends without closing its sockets leaves them here, and each
 * holds a slot and whatever stream, port or mixnet connection it opened.
 * Nothing else frees them, so a client that crashed in a loop filled the
 * table and every later socket on the machine was refused. These are taken
 * out once their owner no longer runs, and the caller releases what they
 * held.
 */

use alloc::vec::Vec;

use super::types::{Socket, Table};

impl Table {
    /// Take out every socket whose owner `alive` says has ended.
    pub fn take_dead(&self, alive: impl Fn(u32) -> bool) -> Vec<Socket> {
        let mut gone = Vec::new();
        let mut g = self.inner.lock();
        for slot in g.iter_mut() {
            if slot.is_some_and(|s| !alive(s.key.pid)) {
                if let Some(s) = slot.take() {
                    gone.push(s);
                }
            }
        }
        gone
    }
}

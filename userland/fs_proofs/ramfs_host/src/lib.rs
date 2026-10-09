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

//! capsule_ramfs as its request loop sees it, for the host: the store, the
//! handle table, the wire codec and the handlers, each from capsule source
//! under the module name the capsule gives it, so every `crate::` path in
//! those files reaches what it reaches in the capsule. Only the receive loop
//! is left out; `serve` stands in for one turn of it.

extern crate alloc;

#[path = "../../../capsule_ramfs/src/handles.rs"]
pub mod handles;
#[path = "../../../capsule_ramfs/src/protocol/mod.rs"]
pub mod protocol;
pub mod server;
#[path = "../../../capsule_ramfs/src/store/mod.rs"]
pub mod store;

use alloc::vec::Vec;

/// The capsule's state, as its receive loop holds it.
pub struct Ramfs {
    pub store: store::Store,
    pub handles: handles::HandleTable,
}

impl Ramfs {
    pub fn new() -> Ramfs {
        Ramfs { store: store::Store::new(), handles: handles::HandleTable::new() }
    }

    /// One request off the wire from `sender_pid`, and the reply, or `None`
    /// for a request the loop drops unanswered.
    pub fn serve(&mut self, msg: &[u8], sender_pid: u32) -> Option<Vec<u8>> {
        let req = protocol::decode_request(msg)?;
        Some(server::dispatch(&mut self.store, &mut self.handles, req, sender_pid))
    }
}

impl Default for Ramfs {
    fn default() -> Self {
        Self::new()
    }
}

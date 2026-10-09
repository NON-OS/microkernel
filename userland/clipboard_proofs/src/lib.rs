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

//! Host proofs for the clipboard service. Every app copies and pastes through
//! it, and any process holding the endpoint can send it any bytes. Everything
//! between the receive and the reply is pure, so the `#[path]` includes pull
//! in the whole of it under the `crate::` paths its files name each other by:
//! the wire, the router and its handlers, and the history they keep.

extern crate alloc;

// The header decode, the reply encoders, the ops, limits and errnos.
#[path = "../../capsule_clipboard/src/protocol/mod.rs"]
pub mod protocol;

// The history: entries, their byte and depth bounds, and the idle wipe. The
// capsule's own lint choice is allowed on the include rather than restyled.
#[allow(clippy::len_without_is_empty)]
#[path = "../../capsule_clipboard/src/state/mod.rs"]
pub mod state;

// The router, its handlers and the reply builders; not the receive loop.
pub mod server;

#[cfg(test)]
mod route_tests;

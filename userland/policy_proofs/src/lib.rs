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

//! Host proofs for the policy service's request path. Every settings read and
//! every first-boot answer goes through it, and any process holding the
//! endpoint can send it any bytes. The `#[path]` includes pull in the real
//! frame decode, handlers, replies and store under the `crate::` paths their
//! files name each other by; libc_shim answers the kernel calls they make and
//! keeps each reply for the tests to read.

// The values a setter writes and a reader gets back, behind one lock.
#[path = "../../capsule_policy/src/store/mod.rs"]
pub mod store;

// What a write tells the kernel about its own settings.
#[path = "../../capsule_policy/src/push/mod.rs"]
pub mod push;

// The frame decode, the handlers and the replies; not the receive loop.
pub mod server;

#[cfg(test)]
mod serve_tests;

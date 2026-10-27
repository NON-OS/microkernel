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

//! Which process owns which address space, read without the manager lock.
//!
//! Every system call's capability check and every context switch asks, and
//! asked under the manager lock they queued behind every map and unmap on
//! the machine, a second and more at boot (regression-30a4b58c4-paging-lock).
//! The manager writes each change here under its lock, so writers never
//! meet; a reader takes each slot whole through its sequence (slot.rs).

mod read;
mod slot;
mod table;
mod write;

pub(crate) use read::{complete, find};
pub(crate) use slot::Entry;
pub(crate) use write::{hand_over, publish, withdraw};

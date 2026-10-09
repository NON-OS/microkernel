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

//! Slow-path IRQ grant records keyed by `grant_id`: the store, the questions
//! bind and teardown ask of it, and the drains revocation takes from it.

mod drain;
mod query;
mod store;

pub(super) use drain::{drain_for_device, drain_for_pid};
pub(super) use query::{count_msix_for_device, has_message_grant, vectors_for_pid};
pub(super) use store::{
    allocate_id, allocate_id_run, insert, insert_many, lookup, remove, vector_for_gsi,
};

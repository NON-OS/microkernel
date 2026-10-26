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
 * A transport lock taken over from a holder killed inside its round trip
 * (L77e): the kernel's rule included by path, a model of a holder, a
 * contender and the kill in every order, and the kernel's glue read.
 */

mod glue_locks_tests;
mod glue_store_tests;
mod glue_tests;
mod glue_whole_tests;
mod tests;
/// The code reader the glue checks share, futex_waiters' among them.
pub(crate) mod text;
/// The kernel's takeover rule, shared with trip_lock: a file is mounted once.
#[path = "../../../../src/services/lifecycle/transport_rule.rs"]
pub(crate) mod transport_rule;
mod world;
mod world_c;
mod world_h;

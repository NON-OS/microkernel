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

//! The virtio 1.0 bring-up, on the shared transport (`nonos_virtio`), split
//! where bus mastering goes on (`setup::transport`): `negotiate` resets the
//! device and settles features, `program` hands it the control queue and
//! lets it run. `restart` runs both again after a reset, on the same grant.

mod negotiate;
mod queue;
mod restart;

pub use negotiate::negotiate;
pub use queue::program;
pub use restart::restart;

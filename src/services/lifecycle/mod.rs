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

// Capsule lifecycle primitives.
//
// Every userland-capsule kernel-side module (entropy, crypto, vfs,
// keyring, ramfs, ...) tracks the capsule's pid + an epoch-style
// generation that bumps on every spawn. Before this module each
// capsule duplicated the same atomics + same liveness check + same
// `mark_dead` logic. They now share `CapsuleState`.
//
// Generation tagging exists so a client request that races a respawn
// returns deterministic ESTALE: the transport captures the generation
// at send time and rejects any reply observed under a different
// generation, even if the request_id happens to match.

mod registry;
mod reply_rest;
mod reply_wait;
mod rest_mark;
mod state;
pub mod supervisor;
pub mod transport;
mod transport_holder;
mod transport_lock;
pub mod transport_nowait;
mod transport_rule;
mod trip_back;
mod trip_guard;
mod trip_lock;
mod trip_rows;
mod trip_turn;
mod waiting;

pub use registry::{register, tick, Capsule};
pub use reply_rest::rest;
pub use state::CapsuleState;
pub use supervisor::{tick as supervisor_tick, Supervised};
pub(crate) use transport_holder::forget as forget_transport_holder;
pub use trip_guard::{TripGuard, TripRead, TripWrite};
pub use trip_lock::TripLock;

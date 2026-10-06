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

//! The session state of net.nym that stands alone: the queue of delivered
//! messages a reader collects from, and the table of sessions clients hold,
//! under the names the capsule's `state` module gives them.

#[path = "../../capsule_net_nym/src/state/rx_queue.rs"]
pub mod rx_queue;

#[path = "../../capsule_net_nym/src/state/gateway.rs"]
pub mod gateway;
#[allow(clippy::new_without_default)]
#[path = "../../capsule_net_nym/src/state/replay.rs"]
pub mod replay;
#[path = "../../capsule_net_nym/src/state/session.rs"]
pub mod session;

/// The session table, file by file, so a proof can hold a table of its own
/// rather than the capsule's one shared `TABLE`. The capsule's own style is
/// kept on the include: ops.rs names `crate::state` without using it, and the
/// table carries lookups its handlers call and these proofs do not.
#[allow(unused_imports, dead_code, clippy::new_without_default)]
#[path = "session_table.rs"]
pub mod table;

pub use gateway::{Gateway, Transport};
pub use session::Session;

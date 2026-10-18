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

//! The virtio init handshake on the legacy register window: `negotiate`
//! resets the device and settles features, then `start` hands it the ring,
//! lets it master the bus (`setup::legacy` passes the broker's command in)
//! and run. A step that does not hold leaves the device marked FAILED.

mod fail;
mod negotiate;
mod program;

pub use negotiate::negotiate;
pub use program::start;

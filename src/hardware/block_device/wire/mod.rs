// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The kernel's one client for every driver on the block wire (PLAN.md,
//! section 4). An endpoint is a capsule the kernel spawns; `Backend::Wire`
//! names it by its index in `table`.

mod codec;
mod decode;
mod endpoint;
mod error;
mod frame;
mod gate;
mod header;
mod identity;
mod layout;
mod leave;
mod map;
mod native;
mod ops;
mod read;
mod row;
mod slow;
mod slow_rule;
mod span;
mod status;
mod table;
mod target;
mod transport;
mod write;

// The sector-to-block map NVMe's client already proves, read from there; it
// moves to block_device/lba_map/ (A14) and this line follows it.
use crate::hardware::nvme_capsule::client::lba_map;
pub use endpoint::{Endpoint, Slot, MAX_UNITS};
pub use error::WireError;
pub use identity::Identity;
pub(in crate::hardware::block_device) use leave::leave_legs;
pub use ops::{healthcheck, identify};
pub(super) use row::{endpoint_of, rows};

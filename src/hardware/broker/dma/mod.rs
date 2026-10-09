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

mod cache;
mod drain;
mod flags;
mod limits;
mod map;
mod placement;
mod pool;
mod records;
mod release;
mod scrub;
mod teardown;
mod types;
mod va;
mod wipe;

pub use map::map_for_caller;
#[cfg(not(target_arch = "x86_64"))]
pub(crate) use pool::low32_default_pages;
pub(crate) use pool::{init_display_pool, init_low32_pool};
#[cfg(target_arch = "x86_64")]
pub(crate) use pool::{low32_fit, low32_target_pages};
pub use release::{release_all_for_pid, release_for_device, unmap_grant};
pub use types::{DmaError, DmaGrant, DmaMapError, DmaMapRequest, DmaMapResult};
pub use wipe::wipe_live_grants;

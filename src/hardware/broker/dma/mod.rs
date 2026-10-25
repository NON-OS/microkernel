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

mod acpi_stop;
mod cache;
pub(crate) mod covering;
mod covers;
mod direction;
mod drain;
mod flags;
mod held_run;
mod kept;
mod kept_rule;
mod kept_settle;
mod limits;
mod map;
mod placement;
mod pool;
mod records;
mod release;
mod scrub;
// Wired at release with the keep path that holds a device's frames until
// its stop is confirmed; the rule and its proofs come first.
#[allow(dead_code)]
mod sdhci_stop;
mod sources;
mod teardown;
mod types;
mod va;
mod wipe;

pub use covers::{covers, covers_any};
pub use held_run::{free_held_run, grant_run_covering};
pub(in crate::hardware::broker) use kept::kept_bytes;
pub(in crate::hardware::broker) use kept_settle::{settle_kept, settle_kept_all};
pub use map::map_for_caller;
pub use pool::init_uncached_pool;
pub(crate) use pool::{init_display_pool, init_low32_pool};
pub(crate) use pool::{low32_fit, low32_target_pages};
pub(crate) use records::grant_ranges;
pub use release::{release_all_for_pid, release_for_device, revoke_for_device, unmap_grant};
pub use types::{DmaError, DmaGrant, DmaMapError, DmaMapRequest, DmaMapResult};
pub use wipe::wipe_live_grants;

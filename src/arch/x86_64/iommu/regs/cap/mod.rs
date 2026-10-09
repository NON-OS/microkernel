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

mod agaw;
mod behaviour;
mod drain;
mod extended;
mod fault;
mod limits;
mod pages;
mod shared;

pub use agaw::{preferred_levels, AgawLevels};
pub use behaviour::{
    caching_mode, has_protected_regions, page_walk_coherent, requires_write_buffer_flush,
};
pub use drain::{read_drain, write_drain};
pub use extended::{
    extended_interrupt_mode, interrupt_remapping, queued_invalidation, snoop_control,
};
pub use fault::{fault_recording_count, fault_recording_offset};
pub use limits::{domain_count, max_address_width};
pub use pages::best_leaf_level;
pub use shared::{all_support, shared_address_width, shared_domain_count, shared_levels};

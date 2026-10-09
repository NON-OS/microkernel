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

//! Queued invalidation (VT-d 3.4, 6.5.2). Units that report ECAP.QI take
//! their invalidations through a descriptor queue; many recent Intel units
//! need it before interrupt remapping can be turned on.

mod descriptor;
mod disable;
mod enable;
mod error;
mod finish;
mod flush;
mod state;
mod submit;

pub use descriptor::{context_global, iec_global, iec_index, iotlb_global, wait, Descriptor};
pub use enable::enable;
pub use flush::{flush_caches, flush_interrupt_entries, flush_iotlb};
pub use state::is_live;
pub use submit::submit;

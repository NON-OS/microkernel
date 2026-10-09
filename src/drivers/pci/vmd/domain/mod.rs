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

//! An Intel Volume Management Device domain, as far as it can be described
//! without touching hardware: which devices are VMDs, where their child buses
//! start, where a child function's config space sits in CFGBAR, and how the
//! buses and memory windows behind the VMD are numbered and assigned.
//!
//! With RST/VMD on in firmware, the NVMe drives (and on some boards the SATA
//! controller) are not on segment 0 at all. They sit in a private PCI domain
//! reached through the VMD endpoint: config space through its BAR0 (CFGBAR),
//! memory through BAR2 (MEMBAR1) and BAR4 (MEMBAR2). Linux's vmd driver is the
//! reference; like it, this numbers the child buses and assigns the child
//! memory itself instead of trusting what the firmware's RST driver left.
//!
//! Pure: everything goes through `ConfigPort`, so the host proofs drive it
//! against a simulated bus.

mod bridge;
mod bridge_window;
mod bus;
mod endpoint;
mod ids;
mod place;
mod port;
mod regs;
mod scan;
mod walk;
mod window;

pub use bus::{bus_count, bus_start, cfg_offset, VMCAP, VMCONFIG};
pub use ids::{is_intel_vmd, INTEL, VMD_DEVICE_IDS};
pub use port::{Assigned, ConfigPort};
pub use walk::assign;
pub use window::{pick_window, Window, MEMBAR2_RESERVED};

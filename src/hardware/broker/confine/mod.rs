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

// The groups below open switches found at boot (L72): pure, every arch.
mod acs_groups;
mod acs_refusal;
mod acs_span;
/// The PCIe ACS walk that finds a device's isolation group: x86_64 only.
#[cfg(target_arch = "x86_64")]
mod acs_walk;
mod attach;
mod avoid;
mod capsule;
mod decide;
mod detach;
mod detach_all;
mod gates;
mod held;
mod interrupts;
mod iova;
mod iova_holes;
mod iova_space;
mod join;
mod join_alias;
mod join_group;
mod map;
mod map_fail;
mod member;
mod one;
mod open;
mod port_only;
mod posture;
mod refuse;
mod reserve;
mod reserve_rmrr;
mod reserved_rule;
mod say;
mod shared;
mod streams;
mod table;
mod unconfined;
mod unmap;
mod unreserve;
mod verdict;
mod virtio;
mod virtio_read;
mod virtio_rule;

pub use acs_groups::set_acs_groups;
pub use acs_span::Port;
pub(super) use attach::{attach, untranslated};
pub(super) use detach::detach;
pub(super) use detach_all::detach_all;
crate::arch::acpi_only! {
    mod held_by;
    pub(super) use held_by::held_by;
}
pub(super) use map::{map, NoAddress};
pub(super) use refuse::Refusal;
pub(super) use unmap::unmap;

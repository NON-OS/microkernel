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

#[cfg(target_arch = "x86_64")]
mod acs;
#[cfg(target_arch = "x86_64")]
mod acs_alias;
#[cfg(target_arch = "x86_64")]
mod acs_kind;
#[cfg(target_arch = "x86_64")]
mod acs_layout;
#[cfg(target_arch = "x86_64")]
mod acs_path;
#[cfg(target_arch = "x86_64")]
mod acs_reach;
#[cfg(target_arch = "x86_64")]
mod acs_read;
mod acs_refusal;
#[cfg(target_arch = "x86_64")]
mod acs_rule;
#[cfg(target_arch = "x86_64")]
mod acs_tree;
#[cfg(target_arch = "x86_64")]
mod acs_words;
mod attach;
mod avoid;
mod capsule;
mod decide;
mod detach;
mod gates;
mod held;
mod held_by;
mod interrupts;
mod iova;
mod iova_space;
mod join;
mod join_alias;
mod join_group;
mod map;
mod member;
mod one;
mod posture;
mod refuse;
mod reserve;
mod say;
mod streams;
mod table;
mod unmap;
mod unreserve;
mod verdict;
mod virtio;
mod virtio_read;
mod virtio_rule;

pub(super) use attach::attach;
pub(super) use detach::{detach, detach_all};
#[allow(unused_imports)] // read by the ACPI broker's SDEV posture, which only x86 builds
pub(super) use held_by::held_by;
pub(super) use map::{map, NoAddress};
pub(super) use refuse::Refusal;
pub(super) use unmap::unmap;

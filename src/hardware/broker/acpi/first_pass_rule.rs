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

//! Who may say the first publish pass is done: the `acpi` service's owner,
//! from any thread of its thread group, as F-33 compares owners. No
//! owner, or anyone else, may not. Pure: `kernel_proofs` mounts it.

use super::same_group::same_holder;

/// `owner`: the registry's pid for `acpi`; `group_of`, the thread group.
pub fn admits(
    owner: Option<u32>,
    caller: u32,
    group_of: impl Fn(u32) -> Option<u32> + Copy,
) -> bool {
    owner.is_some_and(|o| o != 0 && same_holder(o, caller, group_of))
}

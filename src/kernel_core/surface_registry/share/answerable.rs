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

//! Who answers for a surface, as an attach's descriptor names it: its
//! owner, or for a surface pinned from a guest's pages, which the guest
//! owns but did not register, the supervisor that pinned it. Pure, so
//! kernel_proofs::surface_pin holds it by path.

/// `owner`: the group that owns the surface. `supervisor`: the group
/// supervising `owner`, when `owner` is a guest.
pub(crate) fn answerable(owner: u32, supervisor: Option<u32>) -> u32 {
    supervisor.unwrap_or(owner)
}

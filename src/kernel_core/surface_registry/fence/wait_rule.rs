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

//! Who waits on a surface's release fence, pure; kernel_proofs::surface_fence
//! holds it by path. The project that owns the surface; and for a surface
//! pinned from a guest's pages (pin/pin_guest.rs), which the guest owns but
//! can make no call about, the guest's supervisor, which hands the client
//! its buffer back only once the compositor has let it go.

/// `holder`: the waiting group. `owner`: the group that owns the surface.
/// `supervisor`: the group supervising `owner`, when `owner` is a guest.
pub(crate) fn may_wait(holder: u32, owner: u32, supervisor: Option<u32>) -> bool {
    holder != 0 && (holder == owner || supervisor == Some(holder))
}

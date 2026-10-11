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

//! Whether a surface may be placed on another. Pure, so the host proofs
//! hold it.
//!
//! Both must be surfaces of the asking client: the ids a request names are
//! looked up in that client's own object table, so another client's
//! surface, under whatever id, is never a parent. The two must differ, the
//! surface must have no role yet, and the parent must not be placed on the
//! surface, however deep.

use super::restack::placed_on;
use super::state::Subs;

/// `own` says whether an id is one of the client's own surfaces; `shell`,
/// whether `surface` is an xdg_surface's already.
pub fn may_place(
    own: &dyn Fn(u32) -> bool,
    shell: bool,
    subs: &Subs,
    surface: u32,
    parent: u32,
) -> bool {
    surface != parent
        && own(surface)
        && own(parent)
        && !shell
        && subs.of(surface).is_none()
        && !placed_on(subs, parent, surface)
}

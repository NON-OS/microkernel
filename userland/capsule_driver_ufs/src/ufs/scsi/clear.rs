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

//! The data region zeroed before a read: a device that sends less than was
//! asked leaves zeros, never the bytes of an earlier command or unit.

use crate::ufs::ctx::Ctx;

pub(super) fn clear(c: &Ctx, n: usize) {
    let zero = [0u8; 512];
    (0..n).step_by(512).for_each(|at| {
        c.lists.data.put(at, &zero[..(n - at).min(512)]);
    });
}

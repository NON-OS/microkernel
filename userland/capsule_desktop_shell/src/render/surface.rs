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

use crate::state::Context;
use nonos_toolkit::paint::PaintBuffer;

/// A `PaintBuffer` over the shell's backing store, so chrome can draw with the
/// shared primitives instead of raw writes. The buffer aliases the surface, so a
/// caller must finish with one before taking another.
pub fn surface(ctx: &Context) -> PaintBuffer<'_> {
    unsafe { super::layer::layer(ctx.backing_va, ctx.stride, ctx.width, ctx.height) }
}

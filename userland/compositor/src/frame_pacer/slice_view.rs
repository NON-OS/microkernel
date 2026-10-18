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

//! An output's slice of an extended canvas, as the blitter reads it.

use crate::display::layout_rule::Slice;
use crate::state::Context;
use crate::sw_blitter::Surface;

/// The canvas as slice `s` of it: its own origin, the same rows.
pub fn view(ctx: &Context, s: Slice) -> Surface {
    let off = s.x as u64 * 4;
    let byte_len = ctx.backing_len.saturating_sub(off);
    Surface { base_va: ctx.backing_va + off, stride: ctx.stride, width: s.w, height: s.h, byte_len }
}

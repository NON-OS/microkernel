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

//! A drawn buffer handed to the compositor. The chain moves only once the
//! compositor took it: a refused present leaves the shown buffer shown and
//! the drawn one free, out of date as a whole.

use crate::clients::compositor;
use crate::setup::WindowBinding;
use crate::swap::{on_layer, parts_block, with_parts, Damage, Part};

/// How long a frame waits for the compositor to release a buffer: a few
/// of its frames. Past it the frame is given up and painted whole later.
pub(super) const ACQUIRE_MS: u64 = 50;

pub(super) fn present(
    binding: &mut WindowBinding,
    port: u32,
    rid: u32,
    i: usize,
    d: Damage,
) -> bool {
    let pending = binding.swap.present(i, d);
    let taken = compositor::present(port, rid, &pending.body).is_ok();
    binding.swap.settle(pending, taken).is_ok() && taken
}

/// The same with `parts`, boxes inside `d`, told apart: OP_PRESENT_LAYER_PARTS
/// names the layer by the buffer it shows now, and each box is repainted
/// alone. The chain moves by `d`, which holds them all. No parts block for
/// them, the present is `d`'s alone.
pub(super) fn present_parts(
    binding: &mut WindowBinding,
    port: u32,
    rid: u32,
    (i, d): (usize, Damage),
    parts: &[Part],
) -> bool {
    let Some(block) = parts_block(parts) else { return present(binding, port, rid, i, d) };
    let on = binding.surface_handle();
    let pending = binding.swap.present(i, d);
    let body = with_parts(&on_layer(&pending.body, on), &block);
    let taken = compositor::present_layer_parts(port, rid, &body).is_ok();
    binding.swap.settle(pending, taken).is_ok() && taken
}

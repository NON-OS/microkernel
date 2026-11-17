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

//! A shell layer seen as a `PaintBuffer`. The toolkit's `Surface` is
//! implemented for `PaintBuffer`, so the shared widgets draw on the desk and
//! chrome layers through this view, cut to the layer and blended as they are
//! in an app.

use nonos_toolkit::paint::PaintBuffer;

/// The layer at `va`, `stride_bytes` to a row and `width` by `height` pixels.
/// Every primitive is cut to `width` and `height`, so the words past a row's
/// width and past the last row are never written.
///
/// # Safety
///
/// `va` must point at `stride_bytes / 4 * height` writable words that nothing
/// else reads or writes while the buffer is alive.
pub unsafe fn layer<'a>(va: u64, stride_bytes: u32, width: u32, height: u32) -> PaintBuffer<'a> {
    let stride_words = stride_bytes / 4;
    let words = stride_words as usize * height as usize;
    let pixels = unsafe { core::slice::from_raw_parts_mut(va as *mut u32, words) };
    PaintBuffer { pixels, stride_words, width, height }
}

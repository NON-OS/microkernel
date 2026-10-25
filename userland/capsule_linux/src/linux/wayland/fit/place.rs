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

//! Where a window goes on the screen. Pure, so the host proofs hold it.

use super::mode::Configure;

/// Where a `width` by `height` window goes when nothing else places it:
/// centred below the top bar, where a guest's window has always opened.
pub fn centred(width: u32, height: u32) -> (u32, u32) {
    let x = 1920u32.saturating_sub(width) / 2;
    let y = 1080u32.saturating_sub(height).saturating_sub(48) / 2 + 48;
    (x, y)
}

/// The corner a buffer is shown at: the acked configure's rect, else where
/// the window is, else centred.
pub fn origin(
    due: Option<Configure>,
    at: Option<(u32, u32)>,
    width: u32,
    height: u32,
) -> (u32, u32) {
    match due {
        Some(Configure { rect: Some((x, y, _, _)), .. }) => (x, y),
        Some(_) => centred(width, height),
        None => at.unwrap_or_else(|| centred(width, height)),
    }
}

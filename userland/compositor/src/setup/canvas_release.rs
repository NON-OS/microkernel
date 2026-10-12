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

use nonos_libc::mk_munmap;

use crate::state::Context;

/// Give back a canvas that is not the screen, before the screen is replaced.
pub fn release_canvas(ctx: &mut Context) {
    // A doubled canvas and a floor canvas are both their own mapping.
    if ctx.backing_va != ctx.screen.base_va {
        if let Ok(len) = usize::try_from(ctx.backing_len) {
            let _ = mk_munmap(ctx.backing_va as *mut u8, len);
        }
    }
}

/// Give GOP's backing back once its surface is released: it is this
/// capsule's own mapping, and a driver now shows the desktop.
pub fn unmap_backing(va: u64, byte_len: u64) {
    if let Ok(len) = usize::try_from(byte_len) {
        let _ = mk_munmap(va as *mut u8, len);
    }
}

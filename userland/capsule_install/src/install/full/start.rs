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

/*
 * Taking the screen: the heap the image is read into, the surface, and
 * the ground drawn at once, so the screen goes from setup's last frame to
 * this one's colour while the running image is read.
 */

use nonos_libc::{heap_init, mk_debug, mk_exit, HeapError};

use super::surface::{open, Surface};
use crate::install::ui::full::BACKDROP;

pub(super) fn start() -> Surface {
    if let Err(e) = heap_init() {
        if e != HeapError::AlreadyInitialized {
            mk_exit(1);
        }
    }
    super::active::set();
    let s = open().unwrap_or_else(|why| refused(why));
    s.buffer().clear(BACKDROP);
    s.commit(3);
    s
}

fn refused(why: &str) -> ! {
    for part in [&b"[INSTALL] cannot take the screen: "[..], why.as_bytes(), b"\n"] {
        let _ = mk_debug(part.as_ptr(), part.len());
    }
    mk_exit(2)
}

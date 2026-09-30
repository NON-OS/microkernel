/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

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

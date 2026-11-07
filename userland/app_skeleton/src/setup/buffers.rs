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

//! A window's buffers: each mapped, registered and shared with the
//! compositor read only, at one size. All or none: a buffer that fails
//! gives back the ones before it.

use nonos_libc::{mk_munmap, mk_surface_release};

use super::backing::alloc_backing;
use super::binding::{Buffer, BUFFERS};
use super::register::register_and_share;

/// The buffers, each `stride` bytes a row and `byte_len` long.
pub(super) fn open_buffers(w: u32, h: u32) -> Result<([Buffer; BUFFERS], u32, u64), &'static str> {
    let mut bufs = [Buffer { sid: 0, handle: 0, va: 0 }; BUFFERS];
    let (mut stride, mut byte_len) = (0, 0);
    for n in 0..BUFFERS {
        let opened = alloc_backing(w, h).and_then(|(va, s, len)| {
            (stride, byte_len) = (s, len);
            match register_and_share(va, w, h, s, len) {
                Ok((sid, handle)) => Ok(Buffer { sid, handle, va }),
                Err(e) => {
                    let _ = mk_munmap(va as *mut u8, len as usize);
                    Err(e)
                }
            }
        });
        match opened {
            Ok(b) => bufs[n] = b,
            Err(e) => {
                release_buffers(&bufs[..n], byte_len);
                return Err(e);
            }
        }
    }
    Ok((bufs, stride, byte_len))
}

/// Each of `bufs` unshared and unmapped; whether every step took.
pub(super) fn release_buffers(bufs: &[Buffer], byte_len: u64) -> bool {
    bufs.iter().fold(true, |all, b| {
        let released = mk_surface_release(b.handle) >= 0;
        let unmapped = mk_munmap(b.va as *mut u8, byte_len as usize) >= 0;
        all && released && unmapped
    })
}

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

use alloc::boxed::Box;
use core::mem::ManuallyDrop;

use nonos_toolkit::font::ttf::FontRef;

/* One installed page face: its font data, taken out of its Box for as long
as the entry lives, and the face parsed over that data once, at install. A
face borrows its data, so the pair cannot be two ordinary fields. */
pub(super) struct Face {
    pub key: u32,
    font: ManuallyDrop<FontRef<'static>>,
    data: *mut [u8],
}

/* SAFETY: `data` is owned by this entry alone and is only touched in drop;
the registry's lock guards every entry, and the capsule is single threaded. */
unsafe impl Send for Face {}

impl Face {
    /* The parsed face over `bytes`, or None, with the bytes freed, when they
    are not a face. */
    pub fn parse(key: u32, bytes: Box<[u8]>) -> Option<Face> {
        let data = Box::into_raw(bytes);
        /* SAFETY: `data` came from Box::into_raw just above and stays
        allocated until drop frees it, after the face that borrows it. */
        let slice: &'static [u8] = unsafe { &*data };
        match FontRef::try_from_slice(slice) {
            Ok(font) => Some(Face { key, font: ManuallyDrop::new(font), data }),
            Err(_) => {
                /* SAFETY: the failed parse kept no borrow of `data`, and
                this is the only place it is freed. */
                drop(unsafe { Box::from_raw(data) });
                None
            }
        }
    }

    pub fn font(&self) -> &FontRef<'static> {
        &self.font
    }
}

impl Drop for Face {
    fn drop(&mut self) {
        /* SAFETY: the face goes first and is never used again, so nothing
        borrows `data` when its Box is rebuilt; it is freed exactly once. */
        unsafe {
            ManuallyDrop::drop(&mut self.font);
            drop(Box::from_raw(self.data));
        }
    }
}

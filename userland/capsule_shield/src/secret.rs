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

//! The recovery words or key a request carries: wiped when dropped, whether
//! the job ran or was refused.

use crate::answers::borrowed;
use crate::ops;

/* Recovery words and keys: zeroed when dropped, whether the job ran or was refused. */
struct Secret(Vec<String>);

impl Drop for Secret {
    fn drop(&mut self) {
        for f in self.0.iter_mut() {
            // SAFETY: zero bytes are valid UTF-8, so the string stays valid
            // while its bytes are rewritten, and it is dropped right after.
            for b in unsafe { f.as_bytes_mut() } {
                // SAFETY: `b` is a valid, aligned &mut u8 into the string; the
                // volatile write keeps the wipe from being elided.
                unsafe { core::ptr::write_volatile(b, 0) };
            }
        }
    }
}

pub(crate) fn opening(
    fields: Vec<String>,
    open: fn(&[&str]) -> ops::Answer,
) -> impl FnOnce() -> ops::Answer + Send + 'static {
    let secret = Secret(fields);
    move || open(&borrowed(&secret.0))
}

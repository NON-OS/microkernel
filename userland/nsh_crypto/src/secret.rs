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

//! A 32- or 64-byte secret that is wiped when it is dropped. It has no
//! `Clone`, so a secret is never copied by accident; `take` moves bytes in
//! and wipes where they were.

pub struct Secret<const N: usize>([u8; N]);

impl<const N: usize> Secret<N> {
    pub const fn new(bytes: [u8; N]) -> Self {
        Secret(bytes)
    }

    /// Take the bytes from `src` and wipe `src`, so the secret lives in one
    /// place only.
    pub fn take(src: &mut [u8; N]) -> Self {
        let s = Secret(*src);
        nonos_hash::wipe(src);
        s
    }

    pub fn expose(&self) -> &[u8; N] {
        &self.0
    }
}

impl<const N: usize> Drop for Secret<N> {
    fn drop(&mut self) {
        nonos_hash::wipe(&mut self.0);
    }
}

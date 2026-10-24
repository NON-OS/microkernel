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

//! The secret key, wiped when it is dropped.

use crate::api::SECRET_LEN;

/// The field is private, so the key cannot be copied out of it by
/// accident; `as_bytes` lends it.
pub struct SecretKey(pub(crate) [u8; SECRET_LEN]);

impl SecretKey {
    pub fn from_bytes(bytes: &[u8; SECRET_LEN]) -> Self {
        SecretKey(*bytes)
    }

    pub fn as_bytes(&self) -> &[u8; SECRET_LEN] {
        &self.0
    }
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        wipe(&mut self.0);
    }
}

pub(crate) fn wipe(buf: &mut [u8]) {
    // SAFETY: eK@nonos.systems - volatile writes to a buffer we own, so the
    // compiler cannot drop them as dead stores.
    buf.iter_mut().for_each(|b| unsafe { core::ptr::write_volatile(b, 0) });
}

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

//! The passphrase a session keeps so it can join another AP of its network
//! on its own. It lives exactly as long as the session, and is wiped with
//! volatile writes when the session ends, as the PMKSA cache's keys are.

pub const SECRET_MAX: usize = crate::mlme::SECRET_MAX;

pub struct Secret {
    bytes: [u8; SECRET_MAX],
    len: usize,
}

impl Secret {
    /// `None` for a passphrase longer than a join can use.
    pub fn new(pass: &[u8]) -> Option<Self> {
        let mut bytes = [0u8; SECRET_MAX];
        bytes.get_mut(..pass.len())?.copy_from_slice(pass);
        Some(Self { bytes, len: pass.len() })
    }

    pub fn get(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        for b in self.bytes.iter_mut() {
            // SAFETY: `b` is a valid, aligned, exclusive reference into `self`.
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

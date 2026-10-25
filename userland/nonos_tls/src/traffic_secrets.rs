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

//! The secrets of one stage of a connection's keys, and the wipe they get.

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Secrets {
    pub handshake_secret: [u8; 32],
    pub client_secret: [u8; 32],
    pub server_secret: [u8; 32],
    pub client_key: [u8; 32],
    pub client_iv: [u8; 12],
    pub server_key: [u8; 32],
    pub server_iv: [u8; 12],
}

impl Drop for Secrets {
    fn drop(&mut self) {
        for secret in [
            &mut self.handshake_secret,
            &mut self.client_secret,
            &mut self.server_secret,
            &mut self.client_key,
            &mut self.server_key,
        ] {
            wipe(secret);
        }
        wipe(&mut self.client_iv);
        wipe(&mut self.server_iv);
    }
}

/// Zeros that the optimiser may not drop.
pub fn wipe(bytes: &mut [u8]) {
    for byte in bytes.iter_mut() {
        // SAFETY: `byte` is a valid, exclusive reference; the volatile store
        // is what keeps a key from outliving its use.
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

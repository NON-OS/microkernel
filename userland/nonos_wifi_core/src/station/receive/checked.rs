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

//! A frame that passed the checks: its plaintext body (from the LLC/SNAP
//! header on), whether it arrived protected, and its Ethernet addresses.

use super::*;

/// A frame that passed the checks: its plaintext body (from the LLC/SNAP
/// header on), whether it arrived protected, and its Ethernet addresses.
pub(super) struct Checked {
    pub(super) body: Vec<u8>,
    pub(super) protected: bool,
    pub(super) da: [u8; 6],
    pub(super) sa: [u8; 6],
}

impl LinkStation {
    /// Check and convert one received MPDU (FCS already removed).
    /// `hw_decrypted` says the chip decrypted and verified a protected frame
    /// (it leaves the CCMP header and MIC in place).
    pub fn receive(&mut self, mpdu: &[u8], hw_decrypted: bool) -> Rx {
        match self.check(mpdu, hw_decrypted) {
            Ok(c) => self.deliver(&c),
            Err(why) => Rx::Dropped(why),
        }
    }
}

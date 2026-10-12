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

//! The supplicant's own helpers: its key parts and its failure.

use super::kinds::{Failure, State};
use super::Supplicant;

impl Supplicant {
    // KCK: signs EAPOL MICs. KEK: unwraps the group key. Both are slices of the
    // PTK, named here so the handshake code reads like the standard.
    pub(in crate::wpa::supplicant) fn kck(&self) -> &[u8] {
        &self.ptk[0..16]
    }

    pub(in crate::wpa::supplicant) fn kek(&self) -> [u8; 16] {
        let mut k = [0u8; 16];
        k.copy_from_slice(&self.ptk[16..32]);
        k
    }

    pub(in crate::wpa::supplicant) fn fail(&mut self, why: Failure) {
        self.state = State::Failed;
        self.failure = Some(why);
    }
}

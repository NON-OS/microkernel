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

//! The join's secrets, wiped when the state machine is dropped: the
//! passphrase, the randomness, the rekey seed, the PMK and any cached PMKSA
//! offered. The SAE station and the supplicant inside wipe themselves.

use super::*;
use crate::vault::{assert_not_copy, wipe};

assert_not_copy!(Mlme);
assert_not_copy!(Entropy);

impl Drop for Mlme {
    fn drop(&mut self) {
        wipe(&mut self.secret);
        wipe(&mut self.snonce);
        wipe(&mut self.pmk);
        wipe(&mut self.rekey_seed);
        if let Some(pt) = self.pt.as_mut() {
            wipe(pt);
        }
        if let Some(e) = self.sae_entropy.as_mut() {
            wipe(e);
        }
        if let Some(p) = self.cached.as_mut() {
            wipe(&mut p.pmk);
            wipe(&mut p.pmkid);
        }
    }
}

impl Drop for Entropy {
    fn drop(&mut self) {
        wipe(&mut self.snonce);
        wipe(&mut self.sae);
        wipe(&mut self.rekey);
    }
}

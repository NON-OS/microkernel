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

//! The keys a rekey runs under, and the seed of its SNonce. The join's
//! randomness gives the first seed; the driver draws a fresh one after each
//! rekey that finishes, never on an unfinished one, so no frame anyone can
//! send changes it. A draw that fails keeps the old seed, and the count of
//! rekeys finished still keeps every SNonce new.

use super::state::{Supplicant, PTK_MAX};
use crate::vault::wipe;
use crate::wpa::sha256::hmac_sha256_parts;

const LABEL: &[u8] = b"NONOS rekey SNonce";

/// The keys a rekey to one ANonce runs under: our SNonce and the PTK.
pub(super) struct Rekey {
    pub(super) snonce: [u8; 32],
    pub(super) ptk: [u8; PTK_MAX],
}

impl Drop for Rekey {
    fn drop(&mut self) {
        wipe(&mut self.snonce);
        wipe(&mut self.ptk);
    }
}

impl Supplicant {
    // The SNonce and PTK of a rekey to `anonce`; `None` if the PTK could not
    // be derived.
    pub(super) fn rekey_to(&self, anonce: &[u8; 32]) -> Option<Rekey> {
        let n = self.rekeys.to_be_bytes();
        let parts: [&[u8]; 5] = [LABEL, anonce, &self.rekey_seed, &n, &self.snonce];
        let mut r = Rekey { snonce: hmac_sha256_parts(&self.ptk[..32], &parts), ptk: [0; PTK_MAX] };
        let len = 32 + self.pairwise.tk_len();
        let (ids, nonces) = ((&self.aa, &self.spa), (anonce, &r.snonce));
        self.akm.derive_ptk_into(&self.pmk, ids, nonces, &mut r.ptk[..len]).then_some(r)
    }

    /// Take `seed` as the seed of the next rekey's SNonce.
    pub fn reseed_rekey(&mut self, mut seed: [u8; 32]) {
        self.rekey_seed = seed;
        wipe(&mut seed);
    }
}

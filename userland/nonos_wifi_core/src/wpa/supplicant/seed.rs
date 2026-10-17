// NONOS Operating System (AGPL-3.0-or-later)

//! The keys a rekey runs under, and the seed of its SNonce. The join's
//! randomness gives the first seed; the driver draws a fresh one after each
//! rekey that finishes, never on an unfinished one, so no frame anyone can
//! send changes it. A draw that fails keeps the old seed, and the count of
//! rekeys finished still keeps every SNonce new.

use super::state::Supplicant;
use crate::wipe::wipe;
use crate::wpa::sha256::hmac_sha256_parts;

const LABEL: &[u8] = b"NONOS rekey SNonce";

/// The keys a rekey to one ANonce runs under: our SNonce and the PTK.
pub(super) struct Rekey {
    pub(super) snonce: [u8; 32],
    pub(super) ptk: [u8; 48],
}

impl Drop for Rekey {
    fn drop(&mut self) {
        wipe(&mut self.snonce);
        wipe(&mut self.ptk);
    }
}

impl Supplicant {
    // The SNonce and PTK of a rekey to `anonce`.
    pub(super) fn rekey_to(&self, anonce: &[u8; 32]) -> Rekey {
        let n = self.rekeys.to_be_bytes();
        let parts: [&[u8]; 5] = [LABEL, anonce, &self.rekey_seed, &n, &self.snonce];
        let snonce = hmac_sha256_parts(&self.ptk[..32], &parts);
        let ptk = self.akm.derive_ptk(&self.pmk, &self.aa, &self.spa, anonce, &snonce);
        Rekey { snonce, ptk }
    }

    /// Take `seed` as the seed of the next rekey's SNonce.
    pub fn reseed_rekey(&mut self, mut seed: [u8; 32]) {
        self.rekey_seed = seed;
        wipe(&mut seed);
    }
}

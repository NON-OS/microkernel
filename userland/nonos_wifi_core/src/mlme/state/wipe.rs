// NONOS Operating System (AGPL-3.0-or-later)

//! The join's secrets wiped when the machine goes: the passphrase, the PMK,
//! the SNonce and SAE's bytes. The SAE station and the supplicant wipe their
//! own.

use super::fields::Mlme;
use super::request::Entropy;
use crate::wipe::wipe;

impl Drop for Mlme {
    fn drop(&mut self) {
        wipe(&mut self.secret);
        wipe(&mut self.pmk);
        wipe(&mut self.snonce);
        wipe(&mut self.sae_entropy);
    }
}

impl Drop for Entropy {
    fn drop(&mut self) {
        wipe(&mut self.snonce);
        wipe(&mut self.sae);
    }
}

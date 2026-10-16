// NONOS Operating System (AGPL-3.0-or-later)

//! Choosing the AKM, ciphers and PMF a join runs.

use super::super::parse::Rsne;
use super::super::suite::{CIPHER_BIP_CMAC_128, CIPHER_CCMP};
use super::kinds::{Pmf, SelectError, Selection};
use super::policy::JoinPolicy;
use crate::wpa::akm::Akm;

/// Select the AKM and protection for a join to an access point advertising
/// `ap`, whose RSNXE says `ap_h2e` about SAE hash-to-element.
pub fn select(ap: &Rsne, ap_h2e: bool, policy: JoinPolicy) -> Result<Selection, SelectError> {
    if ap.group != CIPHER_CCMP || !ap.pairwise_ccmp {
        return Err(SelectError::UnsupportedCipher);
    }
    if ap.mfpr() && !ap.mfpc() {
        return Err(SelectError::BadProtection);
    }
    // Protection, if negotiated, runs BIP-CMAC-128 for group management frames.
    let bip_ok = ap.group_mgmt.is_none_or(|g| g == CIPHER_BIP_CMAC_128);
    let required = if ap.mfpr() { Pmf::Required } else { Pmf::Capable };

    // WPA3-Personal mandates management frame protection; an access point that
    // offers SAE without being capable of it is misconfigured, not WPA3.
    if ap.akm_sae && policy.allow_sae && ap.mfpc() {
        if !bip_ok {
            return Err(SelectError::BadProtection);
        }
        return Ok(Selection { akm: Akm::Sae, pmf: required, sae_h2e: ap_h2e });
    }
    if !policy.allow_psk {
        // Reached with SAE on offer only when the access point cannot protect
        // management frames, which WPA3 requires.
        if ap.akm_sae && policy.allow_sae {
            return Err(SelectError::BadProtection);
        }
        if ap.akm_psk || ap.akm_psk_sha256 {
            return Err(SelectError::Downgrade);
        }
        return Err(SelectError::UnsupportedAkm);
    }
    if ap.akm_psk_sha256 && ap.mfpc() {
        if !bip_ok {
            return Err(SelectError::BadProtection);
        }
        return Ok(Selection { akm: Akm::PskSha256, pmf: required, sae_h2e: false });
    }
    if ap.akm_psk {
        // Plain PSK negotiates protection only where the access point insists.
        if ap.mfpr() {
            if !bip_ok {
                return Err(SelectError::BadProtection);
            }
            return Ok(Selection { akm: Akm::Psk, pmf: Pmf::Required, sae_h2e: false });
        }
        return Ok(Selection { akm: Akm::Psk, pmf: Pmf::Off, sae_h2e: false });
    }
    Err(SelectError::UnsupportedAkm)
}

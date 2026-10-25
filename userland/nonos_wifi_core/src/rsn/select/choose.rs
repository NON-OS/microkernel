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

//! Select the ciphers, AKM and protection for a join to an access point
//! advertising `ap`, whose RSNXE says `ap_h2e` about SAE hash-to-element.

use super::*;

/// Select the ciphers, AKM and protection for a join to `ap`.
pub fn select(ap: &Rsne, ap_h2e: bool, policy: JoinPolicy) -> Result<Selection, SelectError> {
    let (pairwise, group) = ciphers(ap, policy.ciphers)?;
    let (akm, pmf, sae_h2e) = akm(ap, ap_h2e, policy)?;
    Ok(Selection { akm, pmf, sae_h2e, pairwise, group })
}

fn akm(ap: &Rsne, ap_h2e: bool, policy: JoinPolicy) -> Result<(Akm, Pmf, bool), SelectError> {
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
        return Ok((Akm::Sae, required, ap_h2e));
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
        return Ok((Akm::PskSha256, required, false));
    }
    if ap.akm_psk {
        // Plain PSK negotiates protection only where the access point insists.
        if ap.mfpr() {
            if !bip_ok {
                return Err(SelectError::BadProtection);
            }
            return Ok((Akm::Psk, Pmf::Required, false));
        }
        return Ok((Akm::Psk, Pmf::Off, false));
    }
    Err(SelectError::UnsupportedAkm)
}

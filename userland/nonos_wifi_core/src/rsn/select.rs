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

//! Choose how to join a network from what its RSN element offers and what the
//! person allowed. SAE is taken whenever the access point offers it (a
//! transition-mode network offers SAE and PSK; joining it with PSK would hand
//! out a handshake an eavesdropper can run a dictionary against), PSK-SHA256 or
//! PSK otherwise. A network saved as WPA3 is joined with SAE or not at all, so
//! an access point that answers to the same name with PSK only is refused as a
//! downgrade instead of being trusted. Only CCMP-128 is run: a TKIP group
//! cipher would leave broadcast and multicast frames (ARP, DHCP) undecryptable,
//! so such a network is refused with its own reason.

use super::parse::Rsne;
use super::suite::{CIPHER_BIP_CMAC_128, CIPHER_CCMP};
use crate::wpa::akm::Akm;

/// What the person allowed for this join.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct JoinPolicy {
    /// WPA2-Personal (PSK, PSK-SHA256) may be used. False for a network saved
    /// as WPA3: then only SAE is acceptable.
    pub allow_psk: bool,
    /// WPA3-Personal (SAE) may be used.
    pub allow_sae: bool,
}

impl JoinPolicy {
    /// The default: SAE when offered, PSK otherwise.
    pub const ANY: JoinPolicy = JoinPolicy { allow_psk: true, allow_sae: true };
    /// A network saved as WPA3: SAE only, never downgraded.
    pub const WPA3_ONLY: JoinPolicy = JoinPolicy { allow_psk: false, allow_sae: true };
    /// WPA2-PSK only, for a caller that cannot supply SAE's randomness.
    pub const PSK_ONLY: JoinPolicy = JoinPolicy { allow_psk: true, allow_sae: false };
}

/// How the station will protect management frames.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pmf {
    /// Not negotiated.
    Off,
    /// Negotiated (MFPC set); the access point does not insist on it.
    Capable,
    /// Negotiated and required (MFPC and MFPR set).
    Required,
}

/// The security the join will run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Selection {
    pub akm: Akm,
    pub pmf: Pmf,
    /// For SAE: derive the password element by hash-to-element (the access
    /// point's RSNXE advertises it) rather than hunting and pecking.
    pub sae_h2e: bool,
}

/// Why a network cannot be joined as it advertises itself.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectError {
    /// The group or pairwise cipher is not CCMP-128.
    UnsupportedCipher,
    /// No offered AKM is one this station runs (802.1X/Enterprise, FT, OWE).
    UnsupportedAkm,
    /// The network was saved as WPA3 and now offers only WPA2.
    Downgrade,
    /// The capabilities contradict themselves (MFPR without MFPC) or name a
    /// management cipher other than BIP-CMAC-128 while requiring protection.
    BadProtection,
}

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

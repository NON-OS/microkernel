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

//! The authentication and key management suites the station runs, and what
//! each one changes in the four-way handshake (IEEE Std 802.11-2020, 12.7.3
//! and Table 9-151): which key descriptor version the EAPOL-Key frames carry,
//! which MIC protects them, and how the PTK is expanded from the PMK. The PMK
//! itself comes from the passphrase for the PSK suites and from SAE for SAE.

use super::ptk::{ptk, ptk_sha256};
use crate::eapol::mic::MicKind;

/// An AKM suite from the IEEE 802.11 OUI (00-0F-AC) this station supports.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Akm {
    /// 00-0F-AC:2, WPA2-Personal: HMAC-SHA1 MIC, PRF-SHA1 PTK.
    Psk,
    /// 00-0F-AC:6, PSK with SHA-256: AES-CMAC MIC, KDF-SHA256 PTK.
    PskSha256,
    /// 00-0F-AC:8, WPA3-Personal: the PMK comes from SAE; AES-CMAC MIC,
    /// KDF-SHA256 PTK.
    Sae,
}

impl Akm {
    /// The suite type octet after the 00-0F-AC OUI.
    pub const fn suite_type(self) -> u8 {
        match self {
            Akm::Psk => 2,
            Akm::PskSha256 => 6,
            Akm::Sae => 8,
        }
    }

    /// The key descriptor version the EAPOL-Key Key Information field carries:
    /// 2 for HMAC-SHA1/AES key wrap, 3 for AES-CMAC/AES key wrap, and 0
    /// ("AKM-defined") for SAE, whose MIC is AES-CMAC as well.
    pub const fn key_version(self) -> u16 {
        match self {
            Akm::Psk => 2,
            Akm::PskSha256 => 3,
            Akm::Sae => 0,
        }
    }

    /// The EAPOL-Key MIC algorithm.
    pub const fn mic_kind(self) -> MicKind {
        match self {
            Akm::Psk => MicKind::HmacSha1,
            Akm::PskSha256 | Akm::Sae => MicKind::AesCmac,
        }
    }

    /// The pairwise transient key (KCK || KEK || TK, 16 bytes each).
    pub fn derive_ptk(
        self,
        pmk: &[u8; 32],
        aa: &[u8; 6],
        spa: &[u8; 6],
        anonce: &[u8; 32],
        snonce: &[u8; 32],
    ) -> [u8; 48] {
        match self {
            Akm::Psk => ptk(pmk, aa, spa, anonce, snonce),
            Akm::PskSha256 | Akm::Sae => ptk_sha256(pmk, aa, spa, anonce, snonce),
        }
    }
}

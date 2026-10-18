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

//! What each AKM fixes: its suite type, the key descriptor version and the
//! EAPOL-Key MIC.

use super::*;

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
}

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

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

//! AES-128-CMAC (RFC 4493, NIST SP 800-38B). The EAPOL-Key MIC for the SAE and
//! PSK-SHA256 AKMs is AES-128-CMAC under the KCK (IEEE Std 802.11-2020,
//! 12.7.3, key descriptor version 0 and 3), where the WPA2-PSK MIC was
//! HMAC-SHA1. The message is taken as parts so a frame with its MIC field
//! zeroed is MACed in place. Checked against the RFC 4493 vectors.

/// The CMAC tag length in bytes.
pub const CMAC_LEN: usize = 16;

// Double a block in GF(2^128): shift left one bit, and if the bit shifted out
// was set, fold in the constant 0x87.
pub(super) fn dbl(b: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut carry = 0u8;
    for i in (0..16).rev() {
        out[i] = (b[i] << 1) | carry;
        carry = b[i] >> 7;
    }
    // Branch-free: the mask is all ones exactly when the top bit was set.
    out[15] ^= 0x87 & 0u8.wrapping_sub(carry);
    out
}

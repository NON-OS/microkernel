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

//! The WPA2-Personal pre-shared key from what a person types. IEEE Std
//! 802.11-2020, J.4.1: a passphrase is 8 to 63 printable ASCII characters and
//! the PSK is PBKDF2(passphrase, ssid, 4096, 256); alternatively the PSK itself
//! is entered as 64 hexadecimal digits and used as is. Feeding 64 hex digits to
//! PBKDF2 as if they were a passphrase derives a key no access point holds, so
//! the two are told apart here, and anything else is refused rather than
//! producing a key that can only fail the handshake.

use super::ptk::pmk;

/// The pre-shared key for `secret` on `ssid`, or `None` if `secret` is neither
/// a valid passphrase nor 64 hexadecimal digits.
pub fn psk(secret: &[u8], ssid: &[u8]) -> Option<[u8; 32]> {
    if secret.len() == 64 {
        if let Some(raw) = hex_key(secret) {
            return Some(raw);
        }
    }
    if !(8..=63).contains(&secret.len()) || !secret.iter().all(|&c| (0x20..=0x7E).contains(&c)) {
        return None;
    }
    Some(pmk(secret, ssid))
}

// Decode 64 hexadecimal digits into the 32-byte key, or `None` if any is not one.
fn hex_key(digits: &[u8]) -> Option<[u8; 32]> {
    let mut out = [0u8; 32];
    for (i, pair) in digits.chunks_exact(2).enumerate() {
        out[i] = (nibble(pair[0])? << 4) | nibble(pair[1])?;
    }
    Some(out)
}

fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

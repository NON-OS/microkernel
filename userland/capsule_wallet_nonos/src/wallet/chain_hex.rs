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

//! The hex an address constant is written in, and the NOX address each
//! network takes from nonos_nox_config.

/// A hex digit's value. Any other byte indexes one past the end of a
/// one-element table, which stops the compiler on this line: a const fn
/// cannot return an error, and a sentinel would decode to a plausible byte.
const fn nibble(c: u8) -> u8 {
    let (value, bad) = match c {
        b'0'..=b'9' => (c - b'0', 0),
        b'a'..=b'f' => (c - b'a' + 10, 0),
        b'A'..=b'F' => (c - b'A' + 10, 0),
        _ => (0, 1),
    };
    let refuse = [0u8; 1];
    value | refuse[bad]
}

/// A 20-byte address from its 40 hex digits, checked when the crate builds.
pub const fn hex20(s: &str) -> [u8; 20] {
    let b = s.as_bytes();
    assert!(b.len() == 40);
    let mut out = [0u8; 20];
    let mut i = 0;
    while i < 20 {
        out[i] = (nibble(b[2 * i]) << 4) | nibble(b[2 * i + 1]);
        i += 1;
    }
    out
}

/// NOX on `chain_id`, from nonos_nox_config, the one place it is named (B7).
/// A chain the wallet offers without NOX fails the build.
pub(super) const fn nox_of(chain_id: u64) -> [u8; 20] {
    match nonos_nox_config::address(chain_id, nonos_nox_config::Contract::NoxToken) {
        Ok(a) => a,
        Err(_) => panic!("NOX is configured on every chain the wallet offers"),
    }
}

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

//! A random server name for the link's ClientHello.
//!
//! The relay's address as the server name is an IP literal, which RFC 6066
//! 3 forbids in server_name, and it repeats in clear the address the TCP
//! header already carries. Relays accept any name, and tor's own clients
//! send "www." + 8 to 20 random base32 letters + ".net" (tortls), so this
//! sends the same shape and a link from here reads as any tor client's.

/// The longest name this writes: "www." + 20 letters + ".net".
pub const NAME_MAX: usize = 28;

const ALPHABET: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";

/// Write a name drawn from `seed` into `scratch` and return it. The length
/// of the random part is 8 + seed[0] % 13, so 8 to 20 letters.
pub fn random_name<'a>(scratch: &'a mut [u8; NAME_MAX], seed: &[u8; 21]) -> &'a [u8] {
    let letters = 8 + usize::from(seed[0] % 13);
    scratch[..4].copy_from_slice(b"www.");
    for (slot, byte) in scratch[4..4 + letters].iter_mut().zip(&seed[1..]) {
        *slot = ALPHABET[usize::from(byte & 31)];
    }
    let end = 4 + letters;
    scratch[end..end + 4].copy_from_slice(b".net");
    &scratch[..end + 4]
}

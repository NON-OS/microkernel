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

//! What the status line says when the keyring refuses to make a wallet.
//!
//! The keyring answers with an errno, and each one is a different fact: the
//! caller was not who it said, the store is full, the keyring did not answer,
//! or what was handed to it is not valid. Each is said as itself.

/// The keyring's refusal of generate, import or recover, `invalid` naming
/// what EINVAL means for that request.
pub fn refusal(code: i32, invalid: &'static [u8]) -> &'static [u8] {
    match code {
        -13 => b"the keyring refused this window: it is not the wallet's owner",
        -28 => b"the keyring is full: no room for another wallet",
        -11 => b"the keyring did not answer: try again in a moment",
        -22 => invalid,
        _ => b"the keyring refused, for a reason it did not name",
    }
}

/// What a wallet whose address could not be read came to: it was made in
/// the keyring and taken out again, or it is still held there unused.
pub fn unnamed(forgotten: bool) -> &'static [u8] {
    if forgotten {
        b"the keyring made the wallet but would not name its address, so it was taken back out: nothing changed"
    } else {
        b"the keyring made the wallet but would not name its address, and it is still held there unused"
    }
}

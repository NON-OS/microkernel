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

//! The Debug the kernel's spawn gate withheld (`secret_log`): a capsule that
//! holds keys, input or a secret writes no log off the dev profile, whatever
//! its manifest offers. The process table says when the gate did this, and
//! `audit` and `receipt` say it beside the capsule, so the missing bit never
//! reads as a change to its signed manifest.

use alloc::vec::Vec;

use super::reach::bit;

/// The capabilities that make the gate withhold Debug, in bit order.
pub const SECRET_CAPS: [&[u8]; 4] = [b"InputSource", b"Keyring", b"DeviceSecret", b"InputConsumer"];

/// "holds InputSource" (every secret capability in `caps`, by name).
pub fn holds(caps: u64) -> Vec<u8> {
    let mut out = Vec::from(&b"holds "[..]);
    let held = SECRET_CAPS.iter().filter(|n| caps & bit(n) != 0);
    for (i, n) in held.enumerate() {
        out.extend_from_slice(if i == 0 { b"" } else { b", " });
        out.extend_from_slice(n);
    }
    out
}

/// "debug withheld (holds InputSource)".
pub fn note(caps: u64) -> Vec<u8> {
    let mut out = Vec::from(&b"debug withheld ("[..]);
    out.extend_from_slice(&holds(caps));
    out.push(b')');
    out
}

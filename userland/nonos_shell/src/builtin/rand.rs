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

//! `rand [n]`: n random bytes from the kernel, as hex.
//!
//! The bytes come from the host's `random`, the kernel's own generator
//! (`crypto_random` under the terminal's Crypto capability). The shell keeps
//! no entropy pool and this draws from none: each call asks the kernel afresh.

use alloc::vec;

use crate::flags::parse_usize;
use crate::host::Host;
use crate::out::ShellOut;

/// Bytes drawn when no count is given, and the ceiling on what one call draws.
const DEFAULT: usize = 16;
const MAX: usize = 256;

/// Draw and print the bytes as hex. Returns false on a bad count or a refusal
/// from the kernel, so `rand && ...` gates on a real draw.
pub fn run(out: &mut dyn ShellOut, host: &mut dyn Host, argv: &[&[u8]]) -> bool {
    let count = match argv.get(1) {
        None => DEFAULT,
        Some(arg) => match parse_usize(arg) {
            Some(n) if (1..=MAX).contains(&n) => n,
            _ => {
                out.writeln(b"usage: rand [n]  (1 to 256 bytes)");
                return false;
            }
        },
    };
    let mut buf = vec![0u8; count];
    if !host.random(&mut buf) {
        out.writeln(b"rand: the kernel would not draw randomness");
        return false;
    }
    out.writeln(&super::hex::encode(&buf));
    true
}

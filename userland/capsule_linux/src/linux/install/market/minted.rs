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

//! What the machine writes beside a file it installs, and the order it is
//! written and taken out in, so that a market program is never startable
//! outside its box at its installed path: not half-way through its install,
//! not after a crash, not when an uninstall only partly succeeds. A copy or
//! a loader exec is not held by path. The store is reached through the
//! closures, so the rules are proved on the host.

use alloc::vec::Vec;

/// The trailer the machine mints beside every ELF it installs.
pub const TRAILER: &[u8] = b".zk_trailer.bin";
/// A market program's policy, and the trailer that vouches for it.
pub const POLICY: &[u8] = b".nonos_linux_policy.bin";
pub const POLICY_TRAILER: &[u8] = b".nonos_linux_policy.zk_trailer.bin";
/// What stands beside a held program until its own policy is bound. Nothing
/// vouches for it, and app/bind.rs refuses a program whose policy is there
/// and not vouched for.
pub const PENDING: &[u8] = b"pending: this program's policy is not bound\n";

pub fn beside(at: &[u8], suffix: &[u8]) -> Vec<u8> {
    [at, suffix].concat()
}

/// Write one file through `write`. An ELF of the package a market release
/// names (`held`) gets PENDING beside it first: if that cannot be written,
/// neither is the program.
pub fn place(
    at: &[u8],
    body: &[u8],
    held: bool,
    write: &mut impl FnMut(&[u8], &[u8]) -> bool,
) -> bool {
    if held && body.starts_with(b"\x7fELF") && !write(&beside(at, POLICY), PENDING) {
        return false;
    }
    write(at, body)
}

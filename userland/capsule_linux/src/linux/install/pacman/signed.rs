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

//! A detached signature, checked against the pinned keyring, and said.

use alloc::format;
use alloc::string::String;

use nonos_openpgp::{verify, Key};

use super::verifier::Machine;

pub fn signed(ring: &[Key], sig: &[u8], data: &[u8]) -> bool {
    let (ok, line) = match verify(&Machine, ring, sig, data) {
        Ok(v) => (true, format!("[LINUX] pacman signature Verified by {}\n", hex(&v.fingerprint))),
        Err(r) => (false, format!("[LINUX] pacman signature refused: {}\n", r.why())),
    };
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    ok
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02X}")).collect()
}

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

//! One pacman package: its bytes held to the market's pin when it is the
//! one chosen, to the database's SHA-256, and to its own signature.

use alloc::format;
use alloc::vec::Vec;

use nonos_openpgp::Key;

use super::desc::Record;
use super::source::Source;
use crate::linux::install::auth::{base64, Verified};
use crate::linux::install::pgp::signed;
use crate::linux::install::unpacked::unpacked;

pub fn fetch(
    src: &Source,
    ring: &[Key],
    repo: &str,
    r: &Record,
    pin: Option<&[u8; 32]>,
) -> Option<Verified> {
    let Some(pkg) = src.get(repo, &r.filename) else { return told(r, "it did not download") };
    if pin.is_some_and(|want| blake3::hash(&pkg).as_bytes() != want) {
        refuse(b"[LINUX] package is not the one the market listed\n");
        return told(r, "it is not the one the market listed");
    }
    if Some(nonos_hash::sha256(&pkg)) != r.sha256 {
        refuse(b"[LINUX] package does not match its database record\n");
        return told(r, "its checksum did not match the signed database");
    }
    // The database may carry the signature; a mirror serves it beside the file.
    let sig: Option<Vec<u8>> = match r.pgpsig.is_empty() {
        false => base64::decode(r.pgpsig.as_bytes()),
        true => src.get(repo, &format!("{}.sig", r.filename)),
    };
    let Some(sig) = sig else { return told(r, "its signature did not download") };
    if !signed(ring, &sig, &pkg) {
        return told(r, "its signature did not verify");
    }
    let Some(files) = unpacked(&pkg) else { return told(r, "it did not unpack") };
    Some(Verified::checked(files))
}

fn refuse(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

/// For the person who asked only (refuse::tell): never on the debug line.
fn told(r: &Record, check: &str) -> Option<Verified> {
    crate::linux::install::refuse::tell(&r.name, &r.version, check);
    None
}

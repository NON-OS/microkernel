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
use crate::linux::install::pgp::signed;
use super::source::Source;
use crate::linux::install::unpacked::unpacked;
use crate::linux::install::auth::{base64, Verified};

pub fn fetch(
    src: &Source,
    ring: &[Key],
    repo: &str,
    r: &Record,
    pin: Option<&[u8; 32]>,
) -> Option<Verified> {
    let pkg = src.get(repo, &r.filename)?;
    if pin.is_some_and(|want| blake3::hash(&pkg).as_bytes() != want) {
        return refuse(b"[LINUX] package is not the one the market listed\n");
    }
    if Some(nonos_hash::sha256(&pkg)) != r.sha256 {
        return refuse(b"[LINUX] package does not match its database record\n");
    }
    // The database may carry the signature; a mirror serves it beside the file.
    let sig: Vec<u8> = match r.pgpsig.is_empty() {
        false => base64::decode(r.pgpsig.as_bytes())?,
        true => src.get(repo, &format!("{}.sig", r.filename))?,
    };
    if !signed(ring, &sig, &pkg) {
        return None;
    }
    Some(Verified::checked(unpacked(&pkg)?))
}

fn refuse(line: &[u8]) -> Option<Verified> {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    None
}

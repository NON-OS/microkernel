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

//! One .deb: held to the market's pin when it is the one chosen and to the
//! SHA-256 the signed index gives it, then its data member unpacked.
//! Maintainer scripts in the control member are never run.

use super::ar::members;
use super::packages::Record;
use super::source::Source;
use crate::linux::install::auth::Verified;
use crate::linux::install::unpacked::unpacked;

pub fn fetch(src: &Source, r: &Record, pin: Option<&[u8; 32]>) -> Option<Verified> {
    let deb = src.get(&r.filename)?;
    if pin.is_some_and(|want| blake3::hash(&deb).as_bytes() != want) {
        return refuse(b"[LINUX] package is not the one the market listed\n");
    }
    if Some(nonos_hash::sha256(&deb)) != r.sha256 {
        return refuse(b"[LINUX] package does not match its signed index\n");
    }
    let parts = members(&deb)?;
    if parts.first() != Some(&(b"debian-binary".as_slice(), b"2.0\n".as_slice())) {
        return refuse(b"[LINUX] refused: not a version 2.0 .deb\n");
    }
    let (_, data) = parts.iter().find(|(name, _)| name.starts_with(b"data.tar"))?;
    Some(Verified::checked(unpacked(data)?))
}

fn refuse(line: &[u8]) -> Option<Verified> {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    None
}

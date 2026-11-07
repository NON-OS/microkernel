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
    let Some(deb) = src.get(&r.filename) else { return told(r, "it did not download") };
    if pin.is_some_and(|want| blake3::hash(&deb).as_bytes() != want) {
        refuse(b"[LINUX] package is not the one the market listed\n");
        return told(r, "it is not the one the market listed");
    }
    if Some(nonos_hash::sha256(&deb)) != r.sha256 {
        refuse(b"[LINUX] package does not match its signed index\n");
        return told(r, "its checksum did not match the signed index");
    }
    let Some(parts) = members(&deb) else { return told(r, "it is not a .deb") };
    if parts.first() != Some(&(b"debian-binary".as_slice(), b"2.0\n".as_slice())) {
        refuse(b"[LINUX] refused: not a version 2.0 .deb\n");
        return told(r, "it is not a version 2.0 .deb");
    }
    let data = parts.iter().find(|(name, _)| name.starts_with(b"data.tar"));
    let Some(files) = data.and_then(|(_, d)| unpacked(d)) else {
        return told(r, "it did not unpack");
    };
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

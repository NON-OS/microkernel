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

//! One package, fetched and authenticated.

use nonos_libc::mk_debug;

use super::auth::{verified, Verified};
use super::download::download;
use super::index::Pkg;

/// The package's files, if its bytes authenticate. `pin` is the market's hash
/// of the package the user chose; what it depends on is held to the signed
/// index alone, which names every one of them by checksum.
pub(super) fn fetch(pkg: &Pkg, pin: Option<&[u8; 32]>) -> Option<Verified> {
    let apk = download(&pkg.name, &pkg.version);
    if pin.is_some_and(|want| blake3::hash(&apk).as_bytes() != want) {
        say(b"[LINUX] package is not the one the market listed\n");
        return None;
    }
    let files = pkg.checksum.and_then(|sum| verified(&apk, &sum));
    if files.is_none() {
        say(b"[LINUX] package did not download, or does not match its index record\n");
    }
    files
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}

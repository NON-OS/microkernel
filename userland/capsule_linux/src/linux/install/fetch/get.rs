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

use nonos_inflate::Refused;

use super::super::auth::{verified, Verified};
use super::super::get::package::download;
use super::super::index::Pkg;
use super::super::Why;

/// The package's files, if its bytes authenticate. `pin` is the market's hash
/// of the package the user chose; what it depends on is held to the signed
/// index alone, which names every one of them by checksum.
pub(in crate::linux::install) fn fetch(pkg: &Pkg, pin: Option<&[u8; 32]>) -> Result<Verified, Why> {
    nonos_libc::heap_reset_peak();
    let apk = download(&pkg.name, &pkg.version);
    heap("fetched", &pkg.name);
    if pin.is_some_and(|want| blake3::hash(&apk).as_bytes() != want) {
        say(b"[LINUX] package is not the one the market listed\n");
        return Err(Why::Package);
    }
    let files = pkg.checksum.ok_or(Refused::Bad).and_then(|sum| verified(&apk, &sum));
    drop(apk);
    heap("unpacked", &pkg.name);
    files.map_err(|why| match why {
        Refused::NoMemory(n) => {
            let name = &pkg.name;
            say(alloc::format!(
                "[LINUX] install failed: not enough memory to unpack {name} ({n} bytes)\n"
            )
            .as_bytes());
            Why::UnpackMemory
        }
        Refused::Bad => {
            say(b"[LINUX] package did not download, or does not match its index record\n");
            Why::Package
        }
    })
}

/// How much of the install heap one package took: what is held now, the
/// most held since it began, and the heap's size, in KiB.
fn heap(stage: &str, name: &str) {
    let h = nonos_libc::heap_stats();
    let line = alloc::format!(
        "[LINUX] heap {stage} {name}: used={} peak={} of {} KiB\n",
        h.used >> 10,
        h.peak >> 10,
        h.size >> 10
    );
    say(line.as_bytes());
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}

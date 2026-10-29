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

use alloc::vec::Vec;
use nonos_inflate::Refused;

use super::super::auth::{verified, Verified};
use super::super::index::Pkg;
use super::super::{timing, Why};
use super::told::{say, told, Check};

/// The package's files, if `apk`, its fetched bytes, authenticate. `pin` is
/// the market's hash of the package the user chose; what it depends on is
/// held to the signed index alone, which names every one of them by checksum.
pub(crate) fn fetch(pkg: &Pkg, apk: Vec<u8>, pin: Option<&[u8; 32]>) -> Result<Verified, Why> {
    nonos_libc::heap_reset_peak();
    heap("fetched", &pkg.name);
    let got = !apk.is_empty();
    if pin.is_some_and(|want| blake3::hash(&apk).as_bytes() != want) {
        say(b"[LINUX] package is not the one the market listed\n");
        told(pkg, got, Check::Market);
        return Err(Why::PACKAGE);
    }
    let (h, need) = (nonos_libc::heap_stats(), super::room::needs(pkg.installed));
    let room = (need <= h.size.saturating_sub(h.used)).then_some(()).ok_or(Refused::NoMemory(need));
    let files = timing::verifying(|| {
        room.and(pkg.checksum.ok_or(Refused::Bad)).and_then(|s| verified(&apk, &s))
    });
    drop(apk);
    heap("unpacked", &pkg.name);
    files.map_err(|why| match why {
        Refused::NoMemory(n) => {
            let name = &pkg.name;
            say(alloc::format!(
                "[LINUX] install failed: not enough memory to unpack {name} ({n} bytes)\n"
            )
            .as_bytes());
            told(pkg, got, Check::Memory);
            Why::UNPACK_MEMORY
        }
        Refused::Bad => {
            say(b"[LINUX] package did not download, or does not match its index record\n");
            told(pkg, got, Check::Index);
            Why::PACKAGE
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

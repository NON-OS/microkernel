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

//! Signed programs in a vfs listing. A program is signed when its
//! certificate, manifest and trailer sit beside it: `/capsules/NAME.elf`
//! with `/capsules/NAME.nonos_id_cert.bin` and the rest, or a Linux guest
//! such as `/linux/bin/qwenchat` with `/linux/bin/qwenchat.zk_trailer.bin`
//! (`userland/capsule_linux/src/linux/attest.rs`). A program with a trailer
//! and no certificate was enrolled on this machine alone and is not carried.

use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub(super) const TRAILER: &str = ".zk_trailer.bin";
pub(super) const CERT: &str = ".nonos_id_cert.bin";
pub(super) const MANIFEST: &str = ".manifest.bin";

/// Each signed program in `listing` as its four paths, program first, in
/// path order.
pub(super) fn signed_sets(listing: &[String]) -> Vec<[String; 4]> {
    let has: BTreeSet<&str> = listing.iter().map(String::as_str).collect();
    let mut sets = Vec::new();
    for trailer in &has {
        let Some(base) = trailer.strip_suffix(TRAILER) else { continue };
        let (cert, manifest) = (format!("{base}{CERT}"), format!("{base}{MANIFEST}"));
        if !has.contains(cert.as_str()) || !has.contains(manifest.as_str()) {
            continue;
        }
        let elf = format!("{base}.elf");
        let program = match (has.contains(elf.as_str()), has.contains(base)) {
            (true, _) => elf,
            (false, true) => String::from(base),
            (false, false) => continue,
        };
        sets.push([program, cert, manifest, String::from(*trailer)]);
    }
    sets
}

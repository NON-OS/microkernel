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

//! The suite's index: its Release verified against the pinned keyring, then
//! each component's Packages file held to the checksum the Release gives it.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use nonos_openpgp::{dearmor, Key};

use super::packages::{stanzas, Record};
use super::release::sums;
use super::source::{components, Source};
use crate::linux::install::pgp::signed;
use crate::linux::install::unpacked::decompressed;

const ARCH: &str = "binary-amd64";
const LISTS: [&str; 2] = ["Packages.xz", "Packages.gz"];

/// None if the Release does not verify, or a Packages file does not match it.
pub fn load(src: &Source, ring: &[Key]) -> Option<Vec<Record>> {
    let release = src.get(&format!("dists/{}/Release", src.suite))?;
    let sig = src.get(&format!("dists/{}/Release.gpg", src.suite))?;
    let sig = dearmor(&sig).unwrap_or(sig);
    if !signed(ring, &sig, &release) {
        return None;
    }
    let sums = sums(&String::from_utf8_lossy(&release));
    let mut out = Vec::new();
    for comp in components() {
        let Some((path, sum)) = LISTS.iter().find_map(|l| {
            let path = format!("{comp}/{ARCH}/{l}");
            sums.iter().find(|s| s.path == path).map(|s| (path, s))
        }) else {
            continue;
        };
        let raw = src.get(&format!("dists/{}/{path}", src.suite))?;
        if raw.len() != sum.size || nonos_hash::sha256(&raw) != sum.sha256 {
            say(b"[LINUX] refused: a Packages file does not match its Release\n");
            return None;
        }
        let text = decompressed(&raw)?;
        // The compressed bytes are checked and spent; free them before the
        // parse holds records beside the inflated text.
        drop(raw);
        out.extend(stanzas(&String::from_utf8_lossy(&text)));
    }
    (!out.is_empty()).then_some(out)
}

fn say(line: &[u8]) {
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

/// A package by its own name first, then by a name it provides.
pub fn find<'a>(index: &'a [Record], want: &str) -> Option<&'a Record> {
    let named = index.iter().find(|r| r.name == want);
    named.or_else(|| index.iter().find(|r| r.provides.iter().any(|p| p == want)))
}

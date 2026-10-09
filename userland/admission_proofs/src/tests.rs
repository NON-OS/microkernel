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

//! Every capsule manifest and NONOS-ID certificate committed under
//! `nonos-data/trust/capsules` decodes with the kernel's own decoders.

use crate::{capsule_manifest, nonos_id_cert};
use std::fs;

fn artifacts(suffix: &str) -> Vec<(String, Vec<u8>)> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../nonos-data/trust/capsules");
    let mut out: Vec<_> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(suffix))
        .filter_map(|p| Some((p.display().to_string(), fs::read(&p).ok()?)))
        .collect();
    out.sort();
    out
}

#[test]
fn every_committed_manifest_decodes() {
    let all = artifacts(".manifest.bin");
    assert!(!all.is_empty());
    for (path, bytes) in &all {
        assert!(capsule_manifest::decode::decode(bytes).is_ok(), "{path}");
    }
}

#[test]
fn every_committed_certificate_decodes() {
    let all = artifacts(".nonos_id_cert.bin");
    assert!(!all.is_empty());
    for (path, bytes) in &all {
        assert!(nonos_id_cert::decode::decode(bytes).is_ok(), "{path}");
    }
}

#[test]
fn one_byte_short_is_refused() {
    for (path, bytes) in artifacts(".manifest.bin") {
        let short = &bytes[..bytes.len() - 1];
        assert!(capsule_manifest::decode::decode(short).is_err(), "{path}");
    }
}

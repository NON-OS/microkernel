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

//! The committed release material: every transcript reproduces its root, and
//! every committed capsule trailer verifies, path and STARK, against the
//! release capsule policy root, each in the slot its transcript gives it.

mod common;

use common::*;
use nonos_proof_verify::{read_transcript, release_of, RELEASE};

#[test]
fn every_committed_transcript_reproduces_its_root() {
    for r in &RELEASE {
        let text = String::from_utf8(bytes(&top(&format!("{}.transcript", r.file)))).unwrap();
        let t = read_transcript(&text).unwrap_or_else(|e| panic!("{}: {e}", r.file));
        assert_eq!(t.root, root(&top(r.file)), "{}", r.file);
        assert_eq!(t.root, r.root, "the embedded root is the committed file");
        assert!(t.slots.iter().all(|s| s.kind() == r.kind), "{} holds only its kind", r.name);
        assert!(release_of(&t.root).is_some());
    }
}

fn committed_trailers() -> Vec<String> {
    let dir = trust("capsules");
    let mut v: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path().to_str().unwrap().to_string())
        .filter(|p| p.ends_with(".zk_trailer.bin"))
        .collect();
    v.sort();
    v
}

#[test]
fn every_committed_capsule_trailer_verifies_under_the_release_root() {
    let trailers = committed_trailers();
    assert!(trailers.len() >= 100, "the committed capsule set is in the tree");
    let transcript = trust("policy/zk_capsule_policy_root.bin.transcript");
    let mut args = vec!["transcript", transcript.as_str()];
    args.extend(trailers.iter().map(String::as_str));
    let (code, v) = json(&args);
    assert_eq!(code, 0, "{v}");
    assert_eq!(v["verdict"], "verified");
    assert_eq!(v["release"], true);
    assert_eq!(v["root"]["release"], true);
    assert_eq!(v["root"]["name"], "capsule policy root");
    let results = v["results"].as_array().unwrap();
    assert_eq!(results.len(), trailers.len());
    let mut slots: Vec<u64> = results
        .iter()
        .map(|r| {
            assert_eq!(r["proof"], "stark", "{r}");
            assert_eq!(r["kind"], "capsule");
            r["slot"].as_u64().unwrap()
        })
        .collect();
    slots.sort();
    slots.dedup();
    assert_eq!(slots.len(), trailers.len(), "each trailer opens a slot of its own");
}

#[test]
fn a_committed_trailer_under_another_release_root_fails() {
    let t = committed_trailers().remove(0);
    for file in ["kernel_attest_root.bin", "bootloader_attest_root.bin"] {
        let root = trust(&format!("policy/{file}"));
        let tr = trust("policy/zk_capsule_policy_root.bin.transcript");
        let (code, v) = json(&["transcript", &tr, "--root", &root, &t]);
        assert_eq!(code, 1, "{v}");
        assert_eq!(v["verdict"], "failed");
    }
}

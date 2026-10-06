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

use nonos_capsule_sign::algs::ed25519::Ed25519;
use nonos_capsule_sign::algs::mldsa65::MlDsa65;
use nonos_capsule_sign::algs::traits::{Signer, Verifier};
use nonos_capsule_sign::release::{append_entry, canonical_manifest, find_entry, merkle_root};
use std::process;

fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

fn temp_path(tag: &str) -> String {
    std::env::temp_dir()
        .join(format!("nonos-release-{}-{}.log", tag, process::id()))
        .to_string_lossy()
        .into_owned()
}

#[test]
fn hybrid_release_roundtrip_and_fail_closed() {
    let ed = Ed25519::keygen_random().expect("ed25519 keygen");
    let pq = MlDsa65::keygen_random().expect("mldsa65 keygen");

    let manifest = canonical_manifest(1, &[0xaa; 20], &[0xbb; 32], &[0xcc; 32], &[0xdd; 32], 7);
    let ed_sig = Ed25519::sign(&ed.seed, &manifest).expect("ed25519 sign");
    let pq_sig = MlDsa65::sign(&pq.seed, &manifest).expect("mldsa65 sign");

    let path = temp_path("roundtrip");
    let _ = std::fs::remove_file(&path);
    append_entry(&path, &manifest, &ed_sig, &pq_sig).expect("append");

    let (ed_logged, pq_logged) = find_entry(&path, &to_hex(&manifest)).expect("entry present");
    assert!(Ed25519::verify(&ed.pubkey, &manifest, &ed_logged).unwrap());
    assert!(MlDsa65::verify(&pq.pubkey, &manifest, &pq_logged).unwrap());

    let other_ed = Ed25519::keygen_random().unwrap();
    assert!(!Ed25519::verify(&other_ed.pubkey, &manifest, &ed_logged).unwrap());

    let unlogged = canonical_manifest(2, &[0xaa; 20], &[0xbb; 32], &[0xcc; 32], &[0xdd; 32], 7);
    assert!(find_entry(&path, &to_hex(&unlogged)).is_none());

    let root = merkle_root(&path).expect("merkle root");
    assert_ne!(root, [0u8; 32]);

    std::fs::remove_file(&path).ok();
}

#[test]
fn merkle_root_rejects_corrupt_log() {
    let path = temp_path("corrupt");
    std::fs::write(&path, "not-hex zzz qqq\n").unwrap();
    assert!(merkle_root(&path).is_err());
    std::fs::remove_file(&path).ok();
}

#[test]
fn merkle_root_missing_log_is_empty() {
    let path = temp_path("missing");
    std::fs::remove_file(&path).ok();
    assert_eq!(merkle_root(&path).unwrap(), [0u8; 32]);
}

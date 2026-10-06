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

//! The registrar's rules, each held.

use stark_proofs::crypto::stark::field::Fp;

use crate::native::commit;
use crate::registry::{ek_id, Enrolled, Registry, RegistryError};

fn device(n: u64) -> ([u8; 32], [Fp; 4]) {
    (
        ek_id(&n.to_le_bytes()),
        commit(&[Fp::from_u64(n), Fp::from_u64(n + 1), Fp::from_u64(n + 2), Fp::from_u64(n + 3)]),
    )
}

fn fold(leaf: [Fp; 4], path: &crate::Path) -> [Fp; 4] {
    let h = crate::params::hasher();
    path.siblings.iter().zip(&path.right).fold(leaf, |n, (s, &r)| {
        if r {
            h.compress(s, &n)
        } else {
            h.compress(&n, s)
        }
    })
}

#[test]
fn every_enrolled_device_has_a_path_to_the_root() {
    let mut r = Registry::new(3).expect("depth");
    for n in 0..5 {
        let (k, c) = device(n);
        assert_eq!(r.enroll(k, c), Ok(Enrolled::Added));
    }
    let root = r.root();
    for n in 0..5 {
        let (k, c) = device(n);
        assert_eq!(fold(c, &r.path(&k).expect("path")), root);
    }
}

/// One device, one commitment: a re-enrollment replaces, and the old
/// commitment no longer reaches the root.
#[test]
fn a_reenrollment_replaces_and_never_adds() {
    let mut r = Registry::new(3).expect("depth");
    let (k, old) = device(1);
    r.enroll(k, old).expect("first");
    let (_, new) = device(100);
    assert_eq!(r.enroll(k, new), Ok(Enrolled::Replaced));
    let path = r.path(&k).expect("path");
    assert_eq!(fold(new, &path), r.root());
    assert_ne!(fold(old, &path), r.root());
    assert!(r.transcript().matches("device ").count() == 1);
}

#[test]
fn a_revoked_device_is_out_and_stays_out() {
    let mut r = Registry::new(3).expect("depth");
    let (k, c) = device(1);
    r.enroll(k, c).expect("enroll");
    r.revoke(k);
    assert!(r.path(&k).is_none());
    assert_eq!(r.enroll(k, c), Err(RegistryError::Revoked));
}

#[test]
fn a_commitment_under_a_second_key_is_refused() {
    let mut r = Registry::new(3).expect("depth");
    let (k1, c) = device(1);
    let (k2, _) = device(2);
    r.enroll(k1, c).expect("enroll");
    assert_eq!(r.enroll(k2, c), Err(RegistryError::Duplicate));
}

#[test]
fn a_full_registry_refuses_a_new_device_but_not_a_replacement() {
    let mut r = Registry::new(1).expect("depth");
    let (k1, c1) = device(1);
    let (k2, c2) = device(2);
    let (k3, c3) = device(3);
    r.enroll(k1, c1).expect("1");
    r.enroll(k2, c2).expect("2");
    assert_eq!(r.enroll(k3, c3), Err(RegistryError::Full));
    assert_eq!(r.enroll(k1, c3), Ok(Enrolled::Replaced));
}

#[test]
fn the_root_reproduces_from_the_transcript_and_a_forged_one_is_refused() {
    let mut r = Registry::new(3).expect("depth");
    for n in 0..4 {
        let (k, c) = device(n);
        r.enroll(k, c).expect("enroll");
    }
    r.revoke(device(0).0);
    let text = r.transcript();
    assert_eq!(Registry::recompute(&text).map(|x| x.root()), Ok(r.root()));
    let (k, c) = device(1);
    let line = alloc_line(&k, &c);
    let forged = text.replace(&line, &alloc_line(&k, &device(50).1));
    assert!(Registry::recompute(&forged).is_err());
}

fn alloc_line(k: &[u8; 32], c: &[Fp; 4]) -> String {
    let hex: String = k.iter().map(|x| format!("{x:02x}")).collect();
    format!("device {hex} {} {} {} {}", c[0].to_u64(), c[1].to_u64(), c[2].to_u64(), c[3].to_u64())
}

#[test]
fn a_depth_outside_the_circuit_is_refused() {
    assert!(Registry::new(0).is_err());
    assert!(Registry::new(21).is_err());
}

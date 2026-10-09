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

//! An honest proof verifies; every way of lying about the statement is refused.
//!
//! The tamper cases bypass the prover's own statement check and go straight to
//! the circuit, because the check is a convenience and the circuit is the
//! guarantee: each forged witness must yield no proof that verifies.

use nonos_attest_path::Kind;
use stark_proofs::crypto::stark::field::Fp;

use super::fixture::{boot_ctx, digest, enrolled, fixture, path_at, ENTROPY, SECRET};
use crate::circuit::Forge;
use crate::domain::KIND_BOOTLOADER;
use crate::native::{scope, tag};
use crate::prove::{prove, prove_checked, prove_forged, verify, Error};
use crate::witness::Slot;

fn refused(st: &crate::Statement, w: &crate::Witness) -> bool {
    refused_forged(st, w, Forge::default())
}

fn refused_forged(st: &crate::Statement, w: &crate::Witness, forge: Forge) -> bool {
    !matches!(prove_forged(st, w, &ENTROPY, forge), Ok(p) if verify(st, &p).is_ok())
}

/// The forging path itself proves when what it writes matches the pins, so a
/// refusal below is the pin's doing and not the harness's.
#[test]
fn the_forging_harness_proves_an_honest_witness() {
    let f = fixture();
    let forge =
        Forge { boot_kind: Some(KIND_BOOTLOADER), tag_secret: Some(SECRET.map(Fp::from_u64)) };
    assert!(!refused_forged(&f.st, &f.w, forge));
    assert!(prove_checked(&f.st, &f.w, &ENTROPY).is_ok());
}

#[test]
fn an_enrolled_device_on_an_approved_chain_proves() {
    let f = fixture();
    let p = prove(&f.st, &f.w, &ENTROPY).expect("an honest proof");
    assert_eq!(verify(&f.st, &p), Ok(()));
}

/// The proof is bound to the verifier's context and scope: replayed under
/// another, it fails.
#[test]
fn a_proof_does_not_verify_under_another_context_or_scope() {
    let f = fixture();
    let p = prove(&f.st, &f.w, &ENTROPY).expect("an honest proof");
    let mut other = f.st;
    other.context[0] = Fp::from_u64(43);
    assert!(verify(&other, &p).is_err());
    let mut other = f.st;
    other.scope = scope(b"relayer", 20_000).expect("scope");
    other.tag = tag(&SECRET.map(Fp::from_u64), &other.scope);
    assert!(verify(&other, &p).is_err());
}

/// A kernel slot opened as the bootloader. The attacker writes kind 0 into
/// lane 5, so the walk reaches B; only the pin on lane 5 refuses it.
#[test]
fn a_kernel_slot_cannot_stand_in_for_the_bootloader() {
    let f = fixture();
    let k_ctx = boot_ctx(b"kernel image bytes");
    let (root, tree) = enrolled(Kind::Kernel, std::slice::from_ref(&k_ctx));
    let mut st = f.st;
    st.boot_root = root;
    let mut w = f.w.clone();
    w.bootloader = Slot { digest: digest(&k_ctx), path: path_at(&tree, 0) };
    assert!(matches!(prove(&st, &w, &ENTROPY), Err(Error::Witness(_))));
    assert!(refused_forged(&st, &w, Forge { boot_kind: Some(0), ..Forge::default() }));
}

/// A padding slot of B opened as a bootloader: its digest is public once the
/// transcript is, and the pinned kind is still what refuses it.
#[test]
fn a_padding_slot_cannot_stand_in_for_the_bootloader() {
    let f = fixture();
    let mut b = blake3::Hasher::new();
    b.update(b"NONOS-ATTEST-PATH-PAD-v3");
    b.update(&[0x5a; 32]);
    b.update(&7u32.to_le_bytes());
    let mut w = f.w.clone();
    w.bootloader = Slot { digest: *b.finalize().as_bytes(), path: path_at(&f.boot, 7) };
    assert!(refused_forged(&f.st, &w, Forge { boot_kind: Some(2), ..Forge::default() }));
}

/// A tag from a secret other than the enrolled one. The attacker computes the
/// tag region from that secret, so its root matches the published tag; only
/// the wire to the enrolled secret refuses it.
#[test]
fn a_tag_under_another_secret_is_refused() {
    let f = fixture();
    let other = [Fp::from_u64(9), Fp::from_u64(9), Fp::from_u64(9), Fp::from_u64(9)];
    let mut st = f.st;
    st.tag = tag(&other, &st.scope);
    assert!(refused_forged(&st, &f.w, Forge { tag_secret: Some(other), ..Forge::default() }));
}

/// A device that is not enrolled.
#[test]
fn an_unenrolled_secret_is_refused() {
    let f = fixture();
    let mut w = f.w.clone();
    w.secret[0] = Fp::from_u64(0xdead);
    let mut st = f.st;
    st.tag = tag(&w.secret, &st.scope);
    assert!(refused(&st, &w));
}

/// A kernel that is not in P.
#[test]
fn an_unapproved_kernel_is_refused() {
    let f = fixture();
    let mut w = f.w.clone();
    w.kernel.digest = digest(&boot_ctx(b"a patched kernel"));
    assert!(refused(&f.st, &w));
    let _ = &f.kernel;
}

// NONOS Operating System (AGPL-3.0-or-later)
//! T2 at the gate: an image runs only if its own measurement is enrolled. The
//! forgery is a trailer proving an enrolled slot under a context the forger
//! picked for another image. The private-leaf gate accepts it; the public-leaf
//! gate must refuse it, and the refusal test fails if the gate stops measuring
//! the image itself.

use crate::crypto::stark::air::{
    build_public_trailer, verify_public_trailer, MeasuredSet, Poseidon, RATE,
};
use crate::crypto::stark::attest_params::LOG_ROUNDS;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

pub(crate) const DEPTH: usize = 3;
pub(crate) const ROGUE: &[u8] = b"\x7fELF never enrolled";

pub(crate) fn images() -> Vec<Vec<u8>> {
    (0..1usize << DEPTH).map(|k| alloc::vec![0x7f, b'E', b'L', b'F', k as u8]).collect()
}

pub(crate) fn context(image: &[u8], caps: u64) -> Vec<u8> {
    let mut ctx = blake3::hash(image).as_bytes().to_vec();
    ctx.extend_from_slice(&caps.to_be_bytes());
    ctx
}

pub(crate) fn root_bytes(root: [Fp; RATE]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, lane) in root.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&lane.value().to_le_bytes());
    }
    out
}

pub(crate) fn set(hybrid: bool) -> MeasuredSet {
    let imgs = images();
    let refs: Vec<&[u8]> = imgs.iter().map(|v| v.as_slice()).collect();
    let h = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    if hybrid {
        MeasuredSet::commit_hybrid(&h, &refs)
    } else {
        MeasuredSet::commit(&h, &refs)
    }
}

#[test]
fn an_enrolled_image_verifies() {
    let s = set(true);
    let img = &images()[2];
    let t = build_public_trailer(&s, 2, &context(img, 7)).unwrap_or_default();
    assert!(verify_public_trailer(&root_bytes(s.root()), DEPTH, img, &t, &context(img, 7)));
}

#[test]
fn another_images_slot_does_not_admit_a_rogue() {
    let s = set(true);
    let ctx = context(ROGUE, 7);
    let forged = build_public_trailer(&s, 2, &ctx).unwrap_or_default();
    assert!(!verify_public_trailer(&root_bytes(s.root()), DEPTH, ROGUE, &forged, &ctx));
}

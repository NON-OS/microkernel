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

use super::fixture::{capsule_ctx, enroll, kernel_ctx, policy, DEPTH, EPOCH, PAD_SEED};
use crate::leaf::{leaf_of, Kind};
use crate::poseidon::Poseidon;
use crate::trailer::parse;
use crate::verify::{fold, verify};

#[test]
fn every_enrolled_slot_verifies_at_its_own_context() {
    let (root, tree, cap, kernel) = policy();
    let t0 = tree.trailer(0).expect("trailer 0");
    let t1 = tree.trailer(1).expect("trailer 1");
    assert!(verify(&root, DEPTH, Kind::Capsule, &capsule_ctx(&cap, 0x7, EPOCH), &t0));
    assert!(verify(&root, DEPTH, Kind::Kernel, &kernel_ctx(&kernel, EPOCH), &t1));
}

/*
 * The capability gap, closed. Before this crate the leaf was the image alone and
 * building a trailer needed no secret, so anyone could present an enrolled
 * capsule under any capability word and the gate admitted it. Here the word is
 * in the leaf: the enrolled path, presented with 0xFF, folds to a different root.
 * And no path for 0xFF exists, because no such leaf was ever enrolled.
 */
#[test]
fn an_enrolled_capsule_under_another_capability_word_is_refused() {
    let (root, tree, cap, _) = policy();
    let t0 = tree.trailer(0).expect("trailer 0");
    assert!(!verify(&root, DEPTH, Kind::Capsule, &capsule_ctx(&cap, 0xFF, EPOCH), &t0));
    for slot in 0..(1 << DEPTH) {
        let t = tree.trailer(slot).expect("trailer");
        assert!(!verify(&root, DEPTH, Kind::Capsule, &capsule_ctx(&cap, 0xFF, EPOCH), &t));
    }
}

#[test]
fn another_epoch_is_refused() {
    let (root, tree, cap, kernel) = policy();
    let (t0, t1) = (tree.trailer(0).expect("t0"), tree.trailer(1).expect("t1"));
    assert!(!verify(&root, DEPTH, Kind::Capsule, &capsule_ctx(&cap, 0x7, EPOCH + 1), &t0));
    assert!(!verify(&root, DEPTH, Kind::Kernel, &kernel_ctx(&kernel, EPOCH + 1), &t1));
}

/// A capsule's context presented as a kernel's, and the reverse, never fold.
#[test]
fn the_kind_keeps_kernel_and_capsule_leaves_apart() {
    let (root, tree, cap, kernel) = policy();
    let (t0, t1) = (tree.trailer(0).expect("t0"), tree.trailer(1).expect("t1"));
    assert!(!verify(&root, DEPTH, Kind::Kernel, &capsule_ctx(&cap, 0x7, EPOCH), &t0));
    assert!(!verify(&root, DEPTH, Kind::Capsule, &kernel_ctx(&kernel, EPOCH), &t1));
}

#[test]
fn a_padding_slot_admits_nothing() {
    let (root, tree, _, _) = policy();
    let pad = tree.trailer(5).expect("pad trailer");
    for ctx in [&b""[..], &PAD_SEED[..]] {
        assert!(!verify(&root, DEPTH, Kind::Pad, ctx, &pad));
        assert!(!verify(&root, DEPTH, Kind::Capsule, ctx, &pad));
        assert!(!verify(&root, DEPTH, Kind::Kernel, ctx, &pad));
    }
}

/*
 * The attack the v3 leaf closes. The anonymous proof takes a slot's digest as a
 * private witness and pins only the kind. Under v2 the kind was inside the
 * digest, so whoever knew a pad's digest could fold it as "a kernel" and reach
 * the root. Here the fold starts from the pad's own digest under each other kind,
 * along the pad's own path, and must miss the root every time.
 */
#[test]
fn a_slot_digest_under_another_kind_misses_the_root() {
    let (root, tree, _, _) = policy();
    let h = Poseidon::new();
    let want = crate::trailer::bytes_to_digest(&root).expect("root");
    let slot = 5u32;
    let mut b = blake3::Hasher::new();
    b.update(b"NONOS-ATTEST-PATH-PAD-v3");
    b.update(&PAD_SEED);
    b.update(&slot.to_le_bytes());
    let d = *b.finalize().as_bytes();
    let bytes = tree.trailer(slot as usize).expect("pad trailer");
    let path = parse(&bytes, DEPTH).expect("path");
    assert_eq!(fold(&h, leaf_of(&h, Kind::Pad, &d), &path), Some(want));
    for kind in [Kind::Kernel, Kind::Capsule, Kind::Bootloader] {
        assert_ne!(fold(&h, leaf_of(&h, kind, &d), &path), Some(want));
    }
}

#[test]
fn a_root_built_elsewhere_or_zero_admits_nothing() {
    let (_, tree, cap, _) = policy();
    let t0 = tree.trailer(0).expect("t0");
    let ctx = capsule_ctx(&cap, 0x7, EPOCH);
    let (other, _) = enroll(&[(Kind::Capsule, capsule_ctx(b"other", 0x7, EPOCH))], DEPTH);
    assert!(!verify(&other, DEPTH, Kind::Capsule, &ctx, &t0));
    assert!(!verify(&[0u8; 32], DEPTH, Kind::Capsule, &ctx, &t0));
}

/*
 * The boot tree holds a kernel and a bootloader under the same context shape
 * (measurement and boot epoch). Only the kind keeps them apart, so each slot's
 * path must refuse the other kind with the same bytes.
 */
#[test]
fn a_kernel_and_a_bootloader_with_one_context_stay_apart() {
    let image = b"same bytes, two roles";
    let ctx = kernel_ctx(image, EPOCH);
    let (root, tree) =
        enroll(&[(Kind::Kernel, ctx.clone()), (Kind::Bootloader, ctx.clone())], DEPTH);
    let (t0, t1) = (tree.trailer(0).expect("t0"), tree.trailer(1).expect("t1"));
    assert!(verify(&root, DEPTH, Kind::Kernel, &ctx, &t0));
    assert!(verify(&root, DEPTH, Kind::Bootloader, &ctx, &t1));
    assert!(!verify(&root, DEPTH, Kind::Bootloader, &ctx, &t0));
    assert!(!verify(&root, DEPTH, Kind::Kernel, &ctx, &t1));
}

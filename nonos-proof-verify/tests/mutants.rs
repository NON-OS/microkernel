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

//! Every honest trailer in the fixture passes, and every change to what it
//! is checked for is refused: a flipped byte anywhere, the wrong root, a
//! swapped image, a cut or extended trailer, a changed capability bit, a
//! changed kind, another epoch's context.

mod common;

use common::*;
use nonos_attest_path::{boot_context, capsule_context, Kind};
use nonos_proof_verify::{blake3_of, check_v4, opens, Proof, Refused};

const CAPS_A: u64 = 0x18;
const CAPS_B: u64 = 0x59;

fn capsule_ctx(image: &str, caps: u64) -> Vec<u8> {
    capsule_context(&blake3_of(&bytes(&fx(image))), caps, 1).to_vec()
}

fn kernel_ctx() -> Vec<u8> {
    boot_context(&blake3_of(&bytes(&fx("kernel.image"))), 1).to_vec()
}

fn refused(r: Result<Proof, Refused>) -> Refused {
    r.expect_err("a mutant must be refused")
}

#[test]
fn the_honest_trailers_pass() {
    let caps = root(&fx("caps_root.bin"));
    let a = bytes(&fx("a.trailer"));
    let b = bytes(&fx("b.trailer"));
    assert_eq!(
        check_v4(&caps, Kind::Capsule, &capsule_ctx("a.elf", CAPS_A), &a, false),
        Ok(Proof::Stark)
    );
    assert_eq!(
        check_v4(&caps, Kind::Capsule, &capsule_ctx("b.elf", CAPS_B), &b, false),
        Ok(Proof::Stark)
    );
    let k = root(&fx("kernel_root.bin"));
    assert_eq!(
        check_v4(&k, Kind::Kernel, &kernel_ctx(), &bytes(&fx("kernel.trailer")), false),
        Ok(Proof::Stark)
    );
    assert_eq!(opens(&a).map(|o| o.index), Some(0));
    assert_eq!(opens(&b).map(|o| o.index), Some(1));
}

/// Every header byte, every path byte, and proof bytes spread across the
/// whole proof, each flipped by every one-bit mask in turn of a few.
#[test]
fn a_flipped_byte_anywhere_is_refused() {
    let caps = root(&fx("caps_root.bin"));
    let ctx = capsule_ctx("a.elf", CAPS_A);
    let honest = bytes(&fx("a.trailer"));
    let head = 13 + 266 + 4;
    let mut at: Vec<usize> = (0..head).collect();
    let body = honest.len() - head;
    at.extend((0..256).map(|i| head + i * body / 256));
    at.push(honest.len() - 1);
    for (n, &i) in at.iter().enumerate() {
        let mut t = honest.clone();
        t[i] ^= 1 << (n % 8);
        refused(check_v4(&caps, Kind::Capsule, &ctx, &t, false));
        t[i] = honest[i] ^ 0xff;
        refused(check_v4(&caps, Kind::Capsule, &ctx, &t, false));
    }
}

#[test]
fn the_wrong_root_is_refused() {
    let ctx = capsule_ctx("a.elf", CAPS_A);
    let a = bytes(&fx("a.trailer"));
    for r in [fx("kernel_root.bin"), fx("dev_root.bin"), trust("policy/zk_capsule_policy_root.bin")]
    {
        assert_eq!(
            refused(check_v4(&root(&r), Kind::Capsule, &ctx, &a, false)),
            Refused::Path,
            "{r}"
        );
    }
    let mut near = root(&fx("caps_root.bin"));
    near[0] ^= 1;
    assert_eq!(refused(check_v4(&near, Kind::Capsule, &ctx, &a, false)), Refused::Path);
}

#[test]
fn a_swapped_image_is_refused() {
    let caps = root(&fx("caps_root.bin"));
    let a = bytes(&fx("a.trailer"));
    // B's image with A's capabilities, B's image and capabilities, the kernel's.
    for ctx in [
        capsule_ctx("b.elf", CAPS_A),
        capsule_ctx("b.elf", CAPS_B),
        capsule_ctx("kernel.image", CAPS_A),
    ] {
        assert_eq!(refused(check_v4(&caps, Kind::Capsule, &ctx, &a, false)), Refused::Path);
    }
    // One byte of the image changed.
    let mut img = bytes(&fx("a.elf"));
    img[0] ^= 1;
    let ctx = capsule_context(&blake3_of(&img), CAPS_A, 1);
    assert_eq!(refused(check_v4(&caps, Kind::Capsule, &ctx, &a, false)), Refused::Path);
}

#[test]
fn every_changed_capability_bit_is_refused() {
    let caps = root(&fx("caps_root.bin"));
    let a = bytes(&fx("a.trailer"));
    let m = blake3_of(&bytes(&fx("a.elf")));
    for bit in 0..64 {
        let ctx = capsule_context(&m, CAPS_A ^ (1 << bit), 1);
        assert_eq!(
            refused(check_v4(&caps, Kind::Capsule, &ctx, &a, false)),
            Refused::Path,
            "bit {bit}"
        );
    }
}

#[test]
fn another_epoch_is_refused() {
    let caps = root(&fx("caps_root.bin"));
    let m = blake3_of(&bytes(&fx("a.elf")));
    let a = bytes(&fx("a.trailer"));
    for epoch in [0, 2, u64::MAX] {
        let ctx = capsule_context(&m, CAPS_A, epoch);
        assert_eq!(refused(check_v4(&caps, Kind::Capsule, &ctx, &a, false)), Refused::Path);
    }
    let k = root(&fx("kernel_root.bin"));
    let ctx = boot_context(&blake3_of(&bytes(&fx("kernel.image"))), 2);
    assert_eq!(
        refused(check_v4(&k, Kind::Kernel, &ctx, &bytes(&fx("kernel.trailer")), false)),
        Refused::Path
    );
}

#[test]
fn a_cut_or_extended_trailer_is_refused() {
    let caps = root(&fx("caps_root.bin"));
    let ctx = capsule_ctx("a.elf", CAPS_A);
    let honest = bytes(&fx("a.trailer"));
    let mut cuts: Vec<usize> = (0..300).collect();
    cuts.extend((1..64).map(|i| i * honest.len() / 64));
    cuts.push(honest.len() - 1);
    for n in cuts {
        assert_eq!(
            refused(check_v4(&caps, Kind::Capsule, &ctx, &honest[..n], false)),
            Refused::Malformed,
            "{n}"
        );
    }
    for extra in [&[0u8][..], &[0xff], &honest[..32]] {
        let mut t = honest.clone();
        t.extend_from_slice(extra);
        assert_eq!(refused(check_v4(&caps, Kind::Capsule, &ctx, &t, false)), Refused::Malformed);
    }
}

#[test]
fn another_kind_is_refused() {
    let caps = root(&fx("caps_root.bin"));
    let ctx = capsule_ctx("a.elf", CAPS_A);
    let a = bytes(&fx("a.trailer"));
    // Checked as a kernel or a loader, and with its kind byte rewritten.
    for k in [Kind::Kernel, Kind::Bootloader, Kind::Pad] {
        assert_eq!(refused(check_v4(&caps, k, &ctx, &a, false)), Refused::Malformed);
    }
    for b in [0u8, 2, 3, 4, 0xff] {
        let mut t = a.clone();
        t[8] = b;
        assert_eq!(refused(check_v4(&caps, Kind::Capsule, &ctx, &t, false)), Refused::Malformed);
    }
    // A kernel trailer's proof never stands for a capsule, nor a capsule's for
    // a kernel, even when the path would.
    let k = root(&fx("kernel_root.bin"));
    let kt = bytes(&fx("kernel.trailer"));
    assert_eq!(refused(check_v4(&k, Kind::Capsule, &kernel_ctx(), &kt, false)), Refused::Malformed);
    let mut relabelled = kt.clone();
    relabelled[8] = 3;
    assert_eq!(
        refused(check_v4(&k, Kind::Bootloader, &kernel_ctx(), &relabelled, false)),
        Refused::Path
    );
}

/// The STARK alone is checked too: A's proof spliced onto B's path, which
/// folds for B, is refused by the verifier.
#[test]
fn a_proof_from_another_slot_is_refused() {
    let caps = root(&fx("caps_root.bin"));
    let a = bytes(&fx("a.trailer"));
    let b = bytes(&fx("b.trailer"));
    let (head, path) = (13, 266);
    let mut spliced = b[..head + path].to_vec();
    spliced.extend_from_slice(&a[head + path..]);
    let r = refused(check_v4(&caps, Kind::Capsule, &capsule_ctx("b.elf", CAPS_B), &spliced, false));
    assert!(matches!(r, Refused::Stark(..)), "{r:?}");
    assert_eq!(r.stage(), "stark");
    assert!(r.code() > 400);
}

#[test]
fn a_development_trailer_is_refused_unless_allowed_and_never_a_stark() {
    let dev = root(&fx("dev_root.bin"));
    let ctx = capsule_ctx("a.elf", CAPS_A);
    let a = bytes(&fx("a.dev"));
    assert_eq!(refused(check_v4(&dev, Kind::Capsule, &ctx, &a, false)), Refused::PathOnlyRefused);
    assert_eq!(check_v4(&dev, Kind::Capsule, &ctx, &a, true), Ok(Proof::PathOnly));
    // The same path folds only under its own root and for its own context.
    let caps = root(&fx("caps_root.bin"));
    assert_eq!(refused(check_v4(&caps, Kind::Capsule, &ctx, &a, true)), Refused::Path);
    let other = capsule_ctx("a.elf", CAPS_B);
    assert_eq!(refused(check_v4(&dev, Kind::Capsule, &other, &a, true)), Refused::Path);
    // A v4 trailer's path, lifted out alone, is a development trailer.
    let lifted = bytes(&fx("a.trailer"))[13..13 + 266].to_vec();
    assert_eq!(
        refused(check_v4(&caps, Kind::Capsule, &ctx, &lifted, false)),
        Refused::PathOnlyRefused
    );
}

#[test]
fn a_root_that_is_not_canonical_is_refused_before_any_proof() {
    let ctx = capsule_ctx("a.elf", CAPS_A);
    let a = bytes(&fx("a.trailer"));
    assert_eq!(refused(check_v4(&[0xff; 32], Kind::Capsule, &ctx, &a, false)), Refused::Path);
}

/// Random mutations, deterministic per seed: one to eight bytes changed,
/// inserted or removed anywhere. None passes.
#[test]
fn random_mutants_never_pass() {
    let caps = root(&fx("caps_root.bin"));
    let ctx = capsule_ctx("a.elf", CAPS_A);
    let honest = bytes(&fx("a.trailer"));
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for _ in 0..600 {
        let mut t = honest.clone();
        for _ in 0..1 + rng.below(8) {
            let i = rng.below(t.len());
            match rng.below(3) {
                0 => t[i] ^= 1 + rng.below(255) as u8,
                1 => t.insert(i, rng.next() as u8),
                _ => {
                    t.remove(i);
                }
            }
        }
        if t != honest {
            refused(check_v4(&caps, Kind::Capsule, &ctx, &t, false));
        }
    }
}

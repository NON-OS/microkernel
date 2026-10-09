// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The kernel self-attestation, end to end on the host.
//!
//! It runs the chain the boot path runs: enroll the kernel bytes with
//! nonos-attest-path, embed the trailer with the real footer assembler, parse the
//! footer as the bootloader does, and check the trailer with the same crate the
//! bootloader links. This stands in for a QEMU boot short of running one: the same
//! code, the same byte layout, the same verdict.

use embed_trailer::{assemble_attested_image, SignedKernel};
use nonos_attest_path::{digest_to_bytes, leaf, pad_leaf, verify, Kind, Poseidon, Tree};

// The numbers the bootloader's stark_attest.rs and the enroll tool agree on.
const DEPTH: usize = 8;
const BOOT_EPOCH: u64 = 1;

/// The kernel context, byte for byte what the bootloader builds.
fn kernel_context(kernel_bytes: &[u8]) -> Vec<u8> {
    let mut ctx = blake3::hash(kernel_bytes).as_bytes().to_vec();
    ctx.extend_from_slice(&BOOT_EPOCH.to_be_bytes());
    ctx
}

/// Enroll a kernel in slot 0 of a padded tree, as the enroll tool does.
fn enroll_kernel(kernel_bytes: &[u8]) -> ([u8; 32], Vec<u8>) {
    let h = Poseidon::new();
    let mut leaves: Vec<_> = (0..1u32 << DEPTH).map(|i| pad_leaf(&h, &[0x5a; 32], i)).collect();
    leaves[0] = leaf(&h, Kind::Kernel, &kernel_context(kernel_bytes)).expect("leaf");
    let tree = Tree::commit(&h, &leaves, DEPTH).expect("tree");
    (digest_to_bytes(&tree.root().expect("root")), tree.trailer(0).expect("trailer"))
}

/// The bootloader's footer parse: the kernel region and the proof region.
fn parse_footer(image: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let f = image.len() - embed_trailer::FOOTER_SIZE;
    let u32_at = |o: usize| {
        u32::from_le_bytes([image[f + o], image[f + o + 1], image[f + o + 2], image[f + o + 3]])
            as usize
    };
    let (kernel_size, proof_offset, proof_size) = (u32_at(28), u32_at(40), u32_at(44));
    (image[0..kernel_size].to_vec(), image[proof_offset..proof_offset + proof_size].to_vec())
}

/// The check the bootloader makes before the jump.
fn boot_verify(root: &[u8; 32], kernel_bytes: &[u8], trailer: &[u8]) -> bool {
    verify(root, DEPTH, Kind::Kernel, &kernel_context(kernel_bytes), trailer)
}

fn signed_kernel(kernel_bytes: &[u8]) -> SignedKernel {
    SignedKernel {
        raw_bytes: Vec::new(),
        kernel_bytes: kernel_bytes.to_vec(),
        signature: vec![0u8; 64],
        signature_algorithm: 1,
        rollback_index: 1,
    }
}

const KERNEL: &[u8] = b"nonos-kernel code region, the exact bytes the bootloader measures";

#[test]
fn the_kernel_self_attestation_survives_embed_and_boot_verify() {
    let (root, trailer) = enroll_kernel(KERNEL);
    let image = assemble_attested_image(&signed_kernel(KERNEL), trailer.clone());
    let (parsed_kernel, parsed_proof) = parse_footer(&image.data);
    assert_eq!(parsed_kernel, KERNEL, "the measured kernel region must be unchanged");
    assert_eq!(parsed_proof, trailer, "the path trailer must round-trip through the footer");
    assert!(boot_verify(&root, &parsed_kernel, &parsed_proof), "the enrolled kernel must verify");
}

#[test]
fn a_tampered_kernel_fails_self_attestation() {
    let (root, trailer) = enroll_kernel(KERNEL);
    let mut tampered = KERNEL.to_vec();
    tampered[0] ^= 0x01;
    assert!(!boot_verify(&root, &tampered, &trailer));
}

/*
 * The attacks the image must survive. Each builds a real attested image, mounts
 * the attack, then runs the boot-side parse and check, and asserts the boot is
 * refused. The ISO's threat model, demonstrated rather than asserted.
 */

#[test]
fn attack_flip_a_byte_in_the_image_kernel_region() {
    let (root, trailer) = enroll_kernel(KERNEL);
    let mut image = assemble_attested_image(&signed_kernel(KERNEL), trailer).data;
    image[10] ^= 0xFF;
    let (k, t) = parse_footer(&image);
    assert!(!boot_verify(&root, &k, &t));
}

#[test]
fn attack_swap_a_foreign_kernel_with_a_stolen_trailer() {
    let (root, trailer) = enroll_kernel(b"the genuine enrolled kernel code");
    let foreign = b"a malicious kernel that was never enrolled";
    let (k, t) = parse_footer(&assemble_attested_image(&signed_kernel(foreign), trailer).data);
    assert!(!boot_verify(&root, &k, &t));
}

/*
 * The genuine kernel's path is public: it ships in every image. An attacker who
 * presents it for their own kernel gets nowhere, because the bootloader builds
 * the leaf from the kernel it is about to run and that leaf is not on the path.
 */
#[test]
fn attack_present_the_genuine_path_for_a_foreign_kernel() {
    let (root, genuine_path) = enroll_kernel(b"the genuine enrolled kernel code");
    let foreign = b"a malicious kernel that was never enrolled";
    let image = assemble_attested_image(&signed_kernel(foreign), genuine_path).data;
    let (k, t) = parse_footer(&image);
    assert!(!boot_verify(&root, &k, &t));
}

#[test]
fn attack_forge_a_trailer_under_a_different_root() {
    let (genuine_root, _) = enroll_kernel(b"the genuine enrolled kernel code");
    let attacker = b"the attacker's kernel, enrolled under the attacker's root";
    let (_, attacker_trailer) = enroll_kernel(attacker);
    let (k, t) =
        parse_footer(&assemble_attested_image(&signed_kernel(attacker), attacker_trailer).data);
    assert!(!boot_verify(&genuine_root, &k, &t));
}

/// A STARK trailer from before the path check carries another magic and is
/// refused whole, never read as a path.
#[test]
fn an_old_stark_trailer_is_refused() {
    let (root, mut trailer) = enroll_kernel(KERNEL);
    trailer[..8].copy_from_slice(b"NZKSTRK2");
    assert!(!boot_verify(&root, KERNEL, &trailer));
}

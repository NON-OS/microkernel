// Tests assert by panicking and print what they read, so those lints are off here only.
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use nox_shield_core::notes::{commitment, commitment_bytes};
use nox_shield_core::prover::pool_hasher;
use stark_proofs::shield::note::{note_parts, Note};

// The note commitment against the vector the settlement contract pins. The wallet recomputes
// a commitment to accept a served record and the contract to admit a leaf, so a drifted
// constant, limb order or tree shape would reject every record on a deployed pool. Limbs 1 to
// 11 are value low, value high, asset, four spend key words and four blinding words.
const LIMBS: [u64; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

/// The nested owner digest commitment for LIMBS, word zero first. It pins the Rust reference
/// until the contract's vector for this scheme is confirmed against the deployed contract.
const EXPECTED: [u64; 4] = [
    3_128_788_525_238_172_940,
    977_760_251_882_506_394,
    7_248_597_715_382_424_175,
    11_116_448_888_154_628_040,
];

fn note_of(limbs: [u64; 11]) -> Note {
    Note {
        value: limbs[0] | (limbs[1] << 32),
        asset_id: limbs[2],
        spend_pk: [limbs[3], limbs[4], limbs[5], limbs[6]],
        blinding: [limbs[7], limbs[8], limbs[9], limbs[10]],
    }
}

#[test]
fn the_commitment_matches_the_contracts_own_vector() {
    let hasher = pool_hasher();
    let cm = commitment(&hasher, &note_of(LIMBS));
    let read: [u64; 4] = [cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()];
    // Tied to the pool's own `note_parts`, so a vendor bump that moves the leaf fails here.
    let reference = note_parts(&note_of(LIMBS)).cm;
    assert_eq!(cm, reference, "our commitment drifted from the vendored reference");
    assert_eq!(read, EXPECTED, "the note commitment moved on one of the two sides");
}

// Byte order: a commitment here is four little endian words in word order, sealed against by
// note ciphertexts. The contract packs word zero in the low bits of a big endian bytes32, so
// the wire form is our bytes reversed, stated by this test instead of a document.
#[test]
fn the_wire_order_is_the_reverse_of_ours() {
    let hasher = pool_hasher();
    let cm = commitment(&hasher, &note_of(LIMBS));
    let ours = commitment_bytes(&cm);
    let mut theirs = ours;
    theirs.reverse();
    let packed = "9a458f5dc63e1fc864983137a913d26f0d91b3c4c8f4089a2b6bb0963023bd0c";
    let mut expected = [0u8; 32];
    for (slot, pair) in expected.iter_mut().zip(packed.as_bytes().chunks_exact(2)) {
        let text = core::str::from_utf8(pair).expect("ascii");
        *slot = u8::from_str_radix(text, 16).expect("hex");
    }
    assert_eq!(theirs, expected, "the on-chain packing is not our bytes reversed");
}

// The proven split against the shipped one. `verified/src/limbs.rs` proves the 32 bit split
// round trips, and the vendored prover has its own copy, so this ties the two: a layout
// move in the prover fails here, not on a deployed pool.
#[test]
fn the_provers_limbs_are_the_proven_split() {
    let values =
        [0u64, 1, 0xFFFF_FFFF, 0x1_0000_0000, 0xFFFF_FFFF_FFFF_FFFF, 0x0123_4567_89AB_CDEF];
    for value in values {
        let note = Note { value, asset_id: 0, spend_pk: [0; 4], blinding: [0; 4] };
        let limbs = note.limbs();
        assert_eq!(limbs[0].value(), nox_verified::limbs::low(value), "low half at {value:#x}");
        assert_eq!(limbs[1].value(), nox_verified::limbs::high(value), "high half at {value:#x}");
        assert_eq!(
            nox_verified::limbs::join(limbs[0].value(), limbs[1].value()),
            value,
            "rebuilding at {value:#x}"
        );
    }
}

//! The prover's 32 bit limb split against the one proven in `verified/src/limbs.rs`.

use stark_proofs::shield::note::Note;

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

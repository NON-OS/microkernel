// NONOS Operating System (AGPL-3.0-or-later)
//! Damaged copies of the real blobs: every one is refused or walked to an
//! end, and nothing panics. Overflow checks are on in these runs, so an
//! unchecked add in the parser fails here rather than wrapping.

use crate::arch::fdt::Fdt;
use crate::exercise;
use crate::fixtures::ALL;

fn word(blob: &[u8], field: usize) -> usize {
    u32::from_be_bytes(blob[field * 4..field * 4 + 4].try_into().unwrap()) as usize
}

#[test]
fn every_proper_prefix_of_the_structure_is_refused() {
    for blob in ALL {
        let size = word(blob, 9);
        for cut in (0..size).step_by(4) {
            let mut short = blob.to_vec();
            short[36..40].copy_from_slice(&(cut as u32).to_be_bytes());
            assert!(Fdt::from_bytes(&short).is_err(), "cut at {cut} accepted");
        }
        assert!(Fdt::from_bytes(blob).is_ok());
    }
}

#[test]
fn every_truncated_blob_is_refused() {
    for blob in ALL {
        for len in 0..blob.len() {
            assert!(Fdt::from_bytes(&blob[..len]).is_err());
        }
    }
}

#[test]
fn single_byte_damage_never_panics() {
    for blob in ALL {
        let start = word(blob, 2);
        let end = start + word(blob, 9);
        let mut copy = blob.to_vec();
        // Every byte of the structure block, taking each to the values that
        // most often change meaning: zero, all ones, and one bit flipped.
        for at in start..end {
            let original = copy[at];
            for value in [0x00, 0xff, original ^ 0x01, original ^ 0x80] {
                copy[at] = value;
                exercise::all(&copy);
            }
            copy[at] = original;
        }
    }
}

#[test]
fn header_damage_never_panics() {
    for blob in ALL {
        let mut copy = blob.to_vec();
        for at in 0..40 {
            let original = copy[at];
            for value in [0x00, 0xff, original ^ 0x01, original ^ 0x80] {
                copy[at] = value;
                exercise::all(&copy);
            }
            copy[at] = original;
        }
    }
}

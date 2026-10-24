// NONOS Operating System (AGPL-3.0-or-later)
//! The boot step that hands `/chosen` `rng-seed` to the RNG and zeroes it in
//! the blob: the bytes kept are the seed's, and the erase touches exactly
//! them, leaving a tree that still parses.

use crate::arch::aarch64::boot_entropy::KEPT;
use crate::arch::fdt::Fdt;
use crate::arch::seed_step::{erase, keep};
use crate::fixtures::VIRT_GICV3 as VIRT;

#[test]
fn the_seed_is_kept_then_zeroed_in_place_and_nothing_else_moves() {
    let mut blob = VIRT.to_vec();
    let base = blob.as_mut_ptr();
    let (original, at) = {
        let fdt = Fdt::from_bytes(&blob).unwrap();
        let original = fdt.chosen_bytes(b"rng-seed").unwrap().unwrap().to_vec();
        (original, keep(&fdt, base as u64).unwrap())
    };
    assert_eq!(at.1, 32);
    assert!(KEPT.lock().unwrap().contains(&original));
    assert!(original.iter().any(|b| *b != 0));
    // SAFETY: `at` came from `keep` on this blob, and no reference to it is live.
    unsafe { erase(base, at) };
    let fdt = Fdt::from_bytes(&blob).unwrap();
    assert_eq!(fdt.chosen_bytes(b"rng-seed").unwrap().unwrap(), &[0u8; 32]);
    let (lo, hi) = (at.0 as usize, at.0 as usize + at.1);
    assert_eq!(&blob[..lo], &VIRT[..lo]);
    assert_eq!(&blob[hi..], &VIRT[hi..]);
    assert_eq!(fdt.chosen_bytes(b"kaslr-seed").unwrap().map(<[u8]>::len), Some(8));
}

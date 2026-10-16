// NONOS Operating System (AGPL-3.0-or-later)
//! A commit's rand, scalar, element and PWE, and an exchange's keys, read
//! zero (the identity for points) after the drop.

use core::mem::MaybeUninit;
use core::ptr::{addr_of, drop_in_place, read_volatile};

use p256::{ProjectivePoint, Scalar};

use super::types::{Commit, SaeKeys};

#[test]
fn a_commits_secrets_are_gone_after_it_is_dropped() {
    let c = Commit::new(ProjectivePoint::GENERATOR, &[0x11; 32], &[0x22; 32]).expect("a commit");
    let mut slot = MaybeUninit::new(c);
    let p = slot.as_mut_ptr();
    // SAFETY: `p` points at the Commit `slot` holds, which outlives the reads
    // and is not dropped again; the fields read are plain field values.
    unsafe {
        assert!(read_volatile(addr_of!((*p).rand)) != Scalar::ZERO);
        drop_in_place(p);
        assert!(read_volatile(addr_of!((*p).rand)) == Scalar::ZERO, "rand");
        assert!(read_volatile(addr_of!((*p).scalar)) == Scalar::ZERO, "scalar");
        assert!(read_volatile(addr_of!((*p).element)) == ProjectivePoint::IDENTITY, "element");
        assert!(read_volatile(addr_of!((*p).pwe)) == ProjectivePoint::IDENTITY, "PWE");
    }
}

#[test]
fn an_exchanges_keys_read_zero_after_they_are_dropped() {
    let mut slot = MaybeUninit::new(SaeKeys { kck: [1; 32], pmk: [2; 32], pmkid: [3; 16] });
    let k = slot.as_mut_ptr();
    // SAFETY: as above, for the SaeKeys `slot` holds.
    unsafe {
        drop_in_place(k);
        assert_eq!(read_volatile(addr_of!((*k).kck)), [0; 32]);
        assert_eq!(read_volatile(addr_of!((*k).pmk)), [0; 32]);
        assert_eq!(read_volatile(addr_of!((*k).pmkid)), [0; 16]);
    }
}

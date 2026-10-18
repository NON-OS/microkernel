// NONOS Operating System (AGPL-3.0-or-later)
//! The header and block layout, refused before anything is walked.

use crate::arch::fdt::{Fdt, FdtError};
use crate::blobs::{err, minimal, set};
use crate::fixtures::VIRT_GICV3;

#[test]
fn minimal_tree_is_accepted() {
    let blob = minimal();
    let fdt = Fdt::from_bytes(&blob).unwrap();
    assert_eq!(fdt.header.version, 17);
    assert_eq!(fdt.nodes().count(), 1);
}

#[test]
fn short_or_foreign_blobs_are_refused() {
    assert_eq!(err(&[]), FdtError::OutOfBounds);
    assert_eq!(err(&VIRT_GICV3[..39]), FdtError::OutOfBounds);
    let mut blob = minimal();
    set(&mut blob, 0, 0xfeed_d00d);
    assert_eq!(err(&blob), FdtError::BadMagic);
}

#[test]
fn versions_outside_the_window_are_refused() {
    let mut old = minimal();
    set(&mut old, 5, 16);
    assert_eq!(err(&old), FdtError::BadVersion);
    let mut future = minimal();
    set(&mut future, 6, 17);
    assert_eq!(err(&future), FdtError::BadVersion);
}

#[test]
fn totalsize_must_fit_what_is_there() {
    let mut blob = minimal();
    let len = blob.len() as u32;
    set(&mut blob, 1, len + 4);
    assert_eq!(err(&blob), FdtError::TooLarge);
    let mut big = vec![0u8; 3 << 20];
    big[..blob.len()].copy_from_slice(&blob);
    set(&mut big, 1, 3 << 20);
    assert_eq!(err(&big), FdtError::TooLarge);
    set(&mut blob, 1, 39);
    assert_eq!(err(&blob), FdtError::TotalSizeTooSmall);
}

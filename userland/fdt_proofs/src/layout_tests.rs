// NONOS Operating System (AGPL-3.0-or-later)
//! Block offsets and sizes: inside the blob and at the alignment read at.

use crate::arch::fdt::FdtError;
use crate::blobs::{err, minimal, set};

#[test]
fn blocks_must_sit_inside_and_aligned() {
    let base = minimal();
    let mut past = base.clone();
    set(&mut past, 2, base.len() as u32);
    assert_eq!(err(&past), FdtError::OutOfBounds);
    let mut skew = base.clone();
    set(&mut skew, 2, 54);
    assert_eq!(err(&skew), FdtError::BadLayout);
    let mut ragged = base.clone();
    set(&mut ragged, 9, 15);
    assert_eq!(err(&ragged), FdtError::BadLayout);
    let mut rsv = base.clone();
    set(&mut rsv, 4, 0);
    assert_eq!(err(&rsv), FdtError::OutOfBounds);
    let mut odd = base;
    set(&mut odd, 4, 44);
    assert_eq!(err(&odd), FdtError::BadLayout);
}

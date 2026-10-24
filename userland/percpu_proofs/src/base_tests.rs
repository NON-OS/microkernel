// NONOS Operating System (AGPL-3.0-or-later)
//! A block is found again from its address exactly, and from nothing else:
//! not a byte into a block, not before the first, not past the last.

use crate::percpu::by_base::by_base;
use crate::percpu::operations::{get, BLOCKS};

#[test]
fn each_block_and_only_each_block() {
    let first = get(0).unwrap() as *const _ as u64;
    for i in 0..BLOCKS {
        let at = get(i).unwrap() as *const _ as u64;
        assert!(core::ptr::eq(by_base(at).unwrap(), get(i).unwrap()), "block {i}");
        for off in [1u64, 8, 136, 4095] {
            assert!(by_base(at + off).is_none(), "block {i} + {off}");
        }
    }
    assert!(by_base(first - 4096).is_none());
    assert!(by_base(first + BLOCKS as u64 * 4096).is_none());
    assert!(by_base(0).is_none());
    assert!(by_base(u64::MAX).is_none());
}

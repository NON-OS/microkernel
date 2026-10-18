// NONOS Operating System (AGPL-3.0-or-later)
//! The uncached block's cut from the low pool, over every page-aligned pool
//! base in 8 MiB and every size to 12 MiB: an aligned block inside the pool
//! whenever one fits, the rest one run beside it, never overlapping it, and
//! as large as any block position could leave.

use crate::carve::{split, BLOCK};

const PAGE: u64 = 4096;

fn candidates(base: u64, end: u64) -> Vec<u64> {
    let first = base.div_ceil(BLOCK) * BLOCK;
    (0..).map(|i| first + i * BLOCK).take_while(|b| b + BLOCK <= end).collect()
}

#[test]
fn the_block_is_aligned_inside_and_the_rest_is_the_largest_run() {
    for base in (0..8 << 20).step_by(PAGE as usize) {
        for bytes in (0..12 << 20).step_by(16 * PAGE as usize) {
            let end = base + bytes;
            let all = candidates(base, end);
            let Some((block, rest, rest_bytes)) = split(base, bytes) else {
                assert!(all.is_empty(), "a block fits {base:#x}+{bytes:#x}");
                continue;
            };
            assert!(all.contains(&block), "{block:#x} in {base:#x}+{bytes:#x}");
            let rest_end = rest + rest_bytes;
            assert!(rest >= base && rest_end <= end);
            assert!(rest_end <= block || rest >= block + BLOCK, "rest overlaps the block");
            assert!(rest_end == block || rest == block + BLOCK || rest_bytes == 0);
            let best = all.iter().map(|b| (b - base).max(end - b - BLOCK)).max().unwrap();
            assert_eq!(rest_bytes, best, "{base:#x}+{bytes:#x}");
        }
    }
}

#[test]
fn edges_that_wrap_are_refused() {
    assert_eq!(split(u64::MAX - PAGE, 4 * BLOCK), None);
    assert_eq!(split(u64::MAX - BLOCK, BLOCK), None);
    assert_eq!(split(0, BLOCK - PAGE), None);
    assert_eq!(split(0, BLOCK).map(|(block, _, rest)| (block, rest)), Some((0, 0)));
}

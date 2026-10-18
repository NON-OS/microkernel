//! The pager reads as few pages as a server allows, and never leaves a gap.

use super::paging::paged;
use crate::error::NetError;

/// A server that refuses any page wider than `cap` blocks, and answers each block number it
/// covers as one log.
pub(super) fn server(cap: u64) -> impl FnMut(u64, u64) -> Result<Vec<u64>, NetError> {
    move |from, to| {
        if to - from + 1 > cap {
            Err(NetError::ReplyShape)
        } else {
            Ok((from..=to).collect())
        }
    }
}

#[test]
fn a_server_that_takes_the_whole_span_is_read_in_one_page() {
    let mut calls = 0;
    let mut wide = server(100_000);
    let got = paged(
        1_000,
        58_999,
        100_000,
        10_000,
        |f, t| {
            calls += 1;
            wide(f, t)
        },
        |_| {},
    )
    .expect("read");
    assert_eq!(calls, 1);
    assert_eq!(got, (1_000..=58_999).collect::<Vec<_>>(), "every block, once, in order");
}

#[test]
fn a_refusal_at_the_narrowest_page_fails_the_scan() {
    let mut tiny = server(5_000);
    assert_eq!(paged(0, 30_000, 100_000, 10_000, &mut tiny, |_| {}), Err(NetError::ReplyShape));
}

#[test]
fn the_progress_hears_the_last_block_of_each_page() {
    let mut seen = Vec::new();
    let mut capped = server(50_000);
    paged(0, 99_999, 100_000, 10_000, &mut capped, |to| seen.push(to)).expect("read");
    assert_eq!(seen, [49_999, 99_999]);
}

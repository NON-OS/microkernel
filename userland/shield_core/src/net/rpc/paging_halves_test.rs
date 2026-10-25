//! The pager halves a page only when the server refused it as too wide, and
//! never around a rate limit or a broken connection.

use super::paging::paged;
use super::paging_test::server;
use crate::error::NetError;

#[test]
fn a_server_capped_at_fifty_thousand_is_read_in_halves_without_a_gap() {
    let mut pages = Vec::new();
    let mut capped = server(50_000);
    let got = paged(
        0,
        157_999,
        100_000,
        10_000,
        |f, t| {
            let r = capped(f, t);
            if r.is_ok() {
                pages.push((f, t));
            }
            r
        },
        |_| {},
    )
    .expect("read");
    assert_eq!(got, (0..=157_999).collect::<Vec<_>>());
    assert_eq!(pages.first(), Some(&(0, 49_999)));
    assert!(
        pages.windows(2).all(|w| w[1].0 == w[0].1 + 1),
        "each page starts where the last ended"
    );
    assert_eq!(pages.len(), 4);
}

#[test]
fn a_rate_limit_or_a_broken_connection_is_never_halved_around() {
    for e in [NetError::Rejected { code: 429 }, NetError::Transport] {
        let mut calls = 0;
        let r: Result<Vec<u64>, _> = paged(
            0,
            99_999,
            100_000,
            10_000,
            |_, _| {
                calls += 1;
                Err(e)
            },
            |_| {},
        );
        assert_eq!(r, Err(e));
        assert_eq!(calls, 1, "{e:?} is not retried narrower");
    }
}

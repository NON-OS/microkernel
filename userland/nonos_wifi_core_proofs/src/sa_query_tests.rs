// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! SA Query frames and the station's procedure: retries on the retry timeout
//! with fresh identifiers, any of which a response may echo, and the end of
//! the association at the maximum timeout.

use nonos_wifi_core::pmf::{parse_sa_query, sa_query_frame};
use nonos_wifi_core::pmf::{SaPoll, SaQuery, SaQueryAction, SA_QUERY_MAX_MS, SA_QUERY_RETRY_MS};

#[test]
fn sa_query_frames_carry_their_transaction_identifier() {
    let f = sa_query_frame(
        [2, 0, 0, 0, 0, 2],
        [2, 0, 0, 0, 0, 1],
        0x50,
        SaQueryAction::Request,
        [7, 9],
    );
    assert_eq!(&f[..2], &[0xd0, 0x00], "an Action frame");
    assert_eq!(
        (&f[4..10], &f[10..16], &f[16..22]),
        (&[2, 0, 0, 0, 0, 1][..], &[2, 0, 0, 0, 0, 2][..], &[2, 0, 0, 0, 0, 1][..])
    );
    assert_eq!(&f[24..], &[8, 0, 7, 9]);
    assert_eq!(parse_sa_query(&f[24..]), Some((SaQueryAction::Request, [7, 9])));
    assert_eq!(parse_sa_query(&[8, 1, 1, 2]), Some((SaQueryAction::Response, [1, 2])));
    assert_eq!(parse_sa_query(&[8, 2, 1, 2]), None);
    assert_eq!(parse_sa_query(&[9, 0, 1, 2]), None);
    assert_eq!(parse_sa_query(&[8, 0, 1]), None);
}

#[test]
fn the_procedure_retries_on_its_timeout_and_ends_at_the_maximum() {
    let mut q = SaQuery::start(1000, [1, 1]);
    assert_eq!(q.poll(1000 + SA_QUERY_RETRY_MS - 1, [2, 2]), SaPoll::Wait);
    assert_eq!(q.poll(1000 + SA_QUERY_RETRY_MS, [2, 2]), SaPoll::Retry([2, 2]));
    assert!(q.answers([1, 1]) && q.answers([2, 2]) && !q.answers([3, 3]));
    assert_eq!(q.poll(1000 + SA_QUERY_RETRY_MS + 1, [3, 3]), SaPoll::Wait);
    assert_eq!(q.poll(1000 + SA_QUERY_MAX_MS - 1, [3, 3]), SaPoll::Retry([3, 3]));
    assert_eq!(q.poll(1000 + SA_QUERY_MAX_MS, [4, 4]), SaPoll::Expired);
    // A clock that steps backwards waits rather than underflowing.
    assert_eq!(SaQuery::start(1000, [1, 1]).poll(10, [2, 2]), SaPoll::Wait);
}

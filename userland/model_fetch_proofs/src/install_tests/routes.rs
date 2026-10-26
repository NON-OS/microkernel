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

//! How a model download that found no way out is said: by network, with what
//! to do, never as a cut download.

use super::*;

/*
 * A network that runs but reached no mirror before a byte came is said as
 * not reachable, by name, with what to do; a download that started and
 * stopped still says it goes on from there. Both offer a retry.
 */
#[test]
fn an_unreachable_network_is_named_and_not_a_cut_download() {
    let cases = [
        (NYM_UNREACHABLE, Why::NYM_UNREACHABLE, "Nym is not reachable from this machine"),
        (ANYONE_UNREACHABLE, Why::ANYONE_UNREACHABLE, "Anyone is not reachable from this machine"),
        (MIRROR_UNREACHABLE, Why::MIRROR_UNREACHABLE, "The model mirror is not reachable"),
    ];
    let cut = String::from_utf8_lossy(said(Why::MODEL_FETCH)).into_owned();
    assert!(
        cut.contains("did not finish downloading") && cut.contains("go on from there"),
        "{cut}"
    );
    for (status, why, start) in cases {
        assert_eq!(fetched(i64::from(status)), Err(why));
        let line = String::from_utf8_lossy(said(why)).into_owned();
        assert!(line.starts_with(start), "{line}");
        assert!(line.contains("Check the network, or choose another in Settings"), "{line}");
        assert!(!line.contains("did not finish"), "{line}");
        assert!(Progress::Failed(why.code() as u8).retryable());
    }
    let statuses = [REFUSED, NO_NETWORK, NYM_UNREACHABLE, ANYONE_UNREACHABLE, MIRROR_UNREACHABLE];
    for (i, a) in statuses.iter().enumerate() {
        assert!(statuses[i + 1..].iter().all(|b| a != b));
    }
}

/* Exits that did not answer are their own reason, offer direct, and are not a cut download. */
#[test]
fn no_exit_answering_is_its_own_reason_and_offers_direct() {
    for (status, why, start) in [
        (
            NYM_NO_EXIT,
            Why::NYM_NO_EXIT,
            "No Nym exit answered; try again, or press d to download direct",
        ),
        (ANYONE_NO_EXIT, Why::ANYONE_NO_EXIT, "No Anyone circuit reached the mirror; try again"),
    ] {
        assert_eq!(fetched(i64::from(status)), Err(why));
        let line = String::from_utf8_lossy(said(why)).into_owned();
        assert!(line.starts_with(start), "{line}");
        assert!(line.contains("the mirror sees this machine's address"), "{line}");
        assert!(Progress::Failed(why.code() as u8).retryable());
        assert_ne!(said(why), said(Why::NYM_UNREACHABLE));
        assert_ne!(said(why), said(Why::MODEL_FETCH));
    }
    assert!(crate::store::route_offer::ROUTE_REASONS.contains(&(Why::NYM_NO_EXIT.code() as u8)));
    assert!(crate::store::route_offer::ROUTE_REASONS.contains(&(Why::NYM_UNREACHABLE.code() as u8)));
}

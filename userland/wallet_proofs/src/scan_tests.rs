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

//! The restore scan (v1.4.56): only from its button, one address at a
//! time, stopping after GAP_LIMIT empty indices, resuming on scan further.

use crate::shield::scan::{Scan, ScanState, GAP_LIMIT, MAX_HANDED};

fn batch(from: u32) -> Vec<(u32, [u8; 20])> {
    (from..from + 64).map(|i| (i, [i as u8; 20])).collect()
}

#[test]
fn the_scan_starts_only_from_its_button() {
    let mut s = Scan::default();
    assert_eq!((s.state, s.next()), (ScanState::Idle, None), "nothing read until pressed");
    s.read(Some(5));
    assert!(s.found.is_empty());
    s.start();
    assert_eq!(s.state, ScanState::Deriving(0), "past the accounts");
}

#[test]
fn it_stops_after_the_gap_and_resumes_on_scan_further() {
    let mut s = Scan::default();
    s.start();
    s.batch(batch(3));
    s.read(Some(9));
    assert_eq!(s.found, [(3, [3; 20], 9)]);
    for _ in 0..63 {
        s.read(Some(0));
    }
    assert_eq!(s.state, ScanState::Deriving(67), "the batch ran out first");
    s.batch(batch(67));
    s.read(Some(0));
    assert_eq!((s.state, s.empty_run), (ScanState::Stopped, GAP_LIMIT));
    s.further();
    assert_eq!(s.state, ScanState::Deriving(131));
    s.batch(batch(131));
    let (at, tries) = (s.at, s.tries);
    s.read(None);
    assert_eq!((s.at, s.tries), (at, tries + 1), "no answer: the same address again");
    s.read(None);
    s.read(None);
    assert_eq!((s.at, s.empty_run), (at + 1, 0), "passed over, not counted empty");
    s.batch(vec![]);
    assert_eq!(s.state, ScanState::Stopped, "an empty batch ends it");
    s.batch(vec![(MAX_HANDED, [1; 20])]);
    s.read(Some(0));
    assert_eq!(s.state, ScanState::Stopped, "MAX_HANDED is the end");
    s.further();
    assert_eq!(s.state, ScanState::Stopped);
}

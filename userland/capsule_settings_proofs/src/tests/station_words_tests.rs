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
//! The Network page's words for each address source, and its rows: none
//! when there is nothing to say, one with a note line for Hyper-V's address.

use super::station_tests::station::{rows, said};
use super::station_tests::station_words::HYPER_V;

#[test]
fn each_source_has_its_words() {
    assert_eq!(said(0), None, "not reported: nothing shown");
    assert_eq!(said(1), Some(("Private, new each boot", None)));
    let host = Some(("Assigned by Hyper-V", Some(HYPER_V)));
    assert_eq!((said(2), said(3), said(4)), (host, host, None));
    assert!(HYPER_V.contains("MAC spoofing is on for this VM's adapter"));
}

#[test]
fn no_label_stands_empty() {
    assert_eq!((rows(None), rows(Some(0)), rows(Some(9))), ((0, false), (0, false), (0, false)));
    assert_eq!((rows(Some(1)), rows(Some(2)), rows(Some(3))), ((1, false), (1, true), (1, true)));
}

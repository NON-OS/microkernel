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

use super::request_list::PARAMETER_REQUEST_LIST;
use super::wire::first_discover;

/// Windows 10 and 11's option 55, in order.
const WINDOWS: [u8; 14] = [1, 3, 6, 15, 31, 33, 43, 44, 46, 47, 119, 121, 249, 252];
/// Options that would name the machine or its software.
const NAMING: [u8; 4] = [12, 60, 77, 81];

#[test]
fn the_shipped_list_is_windows_byte_for_byte() {
    assert_eq!(PARAMETER_REQUEST_LIST, WINDOWS);
}

#[test]
fn the_parameter_request_list_is_windows_in_its_order() {
    let (_, list) = first_discover();
    assert_eq!(list, WINDOWS, "option 55 on the wire");
    assert_eq!(list, PARAMETER_REQUEST_LIST, "the shipped constant is what goes out");
}

#[test]
fn nothing_that_names_the_machine_is_sent() {
    let (codes, _) = first_discover();
    for c in NAMING {
        assert!(!codes.contains(&c), "option {c} sent: {codes:?}");
    }
}

#[test]
fn the_options_sent_are_the_ones_the_record_names() {
    // Message type, client id (the per-boot random MAC), maximum message size
    // and the request list; smoltcp's order. What the source comment says
    // differs from Windows, held here so a change to it is seen.
    let (codes, _) = first_discover();
    let mut sorted = codes.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![53, 55, 57, 61], "{codes:?}");
}

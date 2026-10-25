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

//! The Shield's fresh address, read from its answer:
//! forty hex digits from a reply that succeeded, and never zero.

use shield_wire::STATUS_OK;

use crate::order::fresh_reply::{fresh_from, NO_FRESH};

const HEX: &str = "3c3C3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c";

#[test]
fn a_fresh_address_is_forty_hex_digits_with_or_without_0x() {
    assert_eq!(fresh_from(STATUS_OK, &format!("address={HEX}")), Ok([0x3C; 20]));
    let body = format!("status=ok\naddress=0x{HEX}\nmode=1");
    assert_eq!(fresh_from(STATUS_OK, &body), Ok([0x3C; 20]));
}

#[test]
fn no_address_a_failed_answer_a_garbled_or_zero_one_is_refused() {
    let refused = |status: i32, body: &str| assert_eq!(fresh_from(status, body), Err(NO_FRESH));
    refused(STATUS_OK, "");
    refused(STATUS_OK, &format!("addr={HEX}"));
    refused(STATUS_OK + 1, &format!("address={HEX}"));
    refused(STATUS_OK, &format!("address={}", &HEX[..38]));
    refused(STATUS_OK, &format!("address={HEX}00"));
    refused(STATUS_OK, &format!("address=0x{}g", &HEX[..39]));
    refused(STATUS_OK, &format!("address=0x{}", "0".repeat(40)));
}

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

//! Whether a certificate is inside its validity window.

use crate::cert_valid_now::cert_valid_now;
use crate::example_ca::EXAMPLE_CA;
use crate::example_leaf::EXAMPLE_LEAF;
use crate::relay_cert::RELAY_CERT;

/*
 * Times are packed decimal, YYYYMMDDhhmmss, on both sides of the comparison: the
 * clock reader builds one and the certificate parser builds one, and an integer
 * comparison of that shape orders correctly. Fixed values are used here rather
 * than the real clock, so the suite proves the same thing in a year as it does
 * today.
 *
 * The leaf was served on 2026-09-20 and runs from 2026-07-29 to 2026-10-27.
 */
const INSIDE: u64 = 20260920000000;
const BEFORE: u64 = 20260701000000;
const AFTER: u64 = 20261101000000;

#[test]
fn a_current_certificate_is_inside_its_window() {
    assert!(cert_valid_now(EXAMPLE_LEAF, INSIDE));
}
#[test]
fn a_certificate_outside_its_window_is_refused() {
    assert!(!cert_valid_now(EXAMPLE_LEAF, BEFORE), "not yet valid");
    assert!(!cert_valid_now(EXAMPLE_LEAF, AFTER), "expired");
}
#[test]
fn a_zero_clock_refuses_everything() {
    assert!(!cert_valid_now(EXAMPLE_LEAF, 0));
    assert!(!cert_valid_now(EXAMPLE_CA, 0));
    assert!(!cert_valid_now(RELAY_CERT, 0));
}

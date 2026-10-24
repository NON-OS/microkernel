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

//! `check_quote`'s PCR list must be ascending with no repeats, the order
//! a single bank's values are digested in.

use nsh_attest::quote::check_quote;
use nsh_attest::Refusal;

use super::quote_v3::setup;

#[test]
fn a_pcr_list_out_of_order_or_repeated_is_refused() {
    let (values, attest, sig, ak) = setup();
    let refs: Vec<&[u8; 32]> = values.iter().collect();
    for list in [[0, 2, 1, 7], [0, 1, 2, 2]] {
        let r = check_quote(&attest, &sig, &ak, &[0x42; 32], &list, &refs);
        assert!(r.is_err(), "{list:?}");
    }
    let r = check_quote(&attest, &sig, &ak, &[0x42; 32], &[0, 2, 1, 7], &refs);
    assert_eq!(r.map(|_| ()), Err(Refusal::Selection));
    let five: Vec<&[u8; 32]> = values.iter().chain([&values[3]]).collect();
    let r = check_quote(&attest, &sig, &ak, &[0x42; 32], &[0, 1, 2, 7, 7], &five);
    assert_eq!(r.map(|_| ()), Err(Refusal::Selection), "a repeat with the same bitmap");
}

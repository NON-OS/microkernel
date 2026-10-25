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

/*
 * The checks the Shield send screen runs before it asks the service, on
 * the pinned payee's nox1 text that shield-core itself produces (its test
 * the_text_is_the_one_the_nonos_wallet_checks reads the same file).
 */

use super::shield_fixtures::PINNED;
use crate::shield_typed::address_ok;

#[test]
fn shield_cores_own_address_text_passes() {
    assert_eq!(PINNED.len(), 2009);
    assert!(address_ok(PINNED));
}

/* An address the NONOS shield app made on an iPhone (0.9.0, Sepolia). */
const PHONE: &str = include_str!("../fixtures/nox1-phone.txt");

#[test]
fn an_address_the_phone_app_made_passes() {
    assert_eq!(PHONE.len(), 2009);
    assert!(address_ok(PHONE));
}

#[test]
fn a_cut_or_padded_address_is_refused() {
    assert!(!address_ok(&PINNED[..2008]));
    assert!(!address_ok(&alloc::format!("{PINNED}a")));
    assert!(!address_ok("nox1"));
    assert!(!address_ok(""));
}

#[test]
fn a_letter_outside_the_alphabet_is_refused() {
    for bad in ['1', '8', '9', '0', 'A', ' ', '.'] {
        let mut t = alloc::string::String::from(PINNED);
        t.replace_range(100..101, &bad.to_string());
        assert!(!address_ok(&t), "{bad:?} slipped through");
    }
}

#[test]
fn another_prefix_is_refused() {
    let t = alloc::format!("nox2{}", &PINNED[4..]);
    assert!(!address_ok(&t));
    let t = alloc::format!("0x{}", &PINNED[2..]);
    assert!(!address_ok(&t));
}

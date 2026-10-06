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

use crate::shield_typed::{address_ok, amount_ok, pasted_address, reads_typed, tail_start};

const PINNED: &str = include_str!("../fixtures/nox1-pinned.txt");

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

#[test]
fn amounts_are_positive_decimals_of_at_most_18_places() {
    for ok in ["1", "0.02", ".5", "5.", "2000", "0.000000000000000001"] {
        assert!(amount_ok(ok), "{ok} refused");
    }
    for bad in [
        "",
        ".",
        "0",
        "0.0",
        "00.000",
        "1.2.3",
        "-1",
        "1e3",
        "1,000",
        " 1",
        "0.0000000000000000001",
    ] {
        assert!(!amount_ok(bad), "{bad} accepted");
    }
}

/* The screens as state/shield_ui.rs numbers them. */
const DEPOSIT: u8 = 1;
const SEND: u8 = 2;
const WITHDRAW: u8 = 3;
const REVIEW: u8 = 4;

#[test]
fn only_a_send_or_its_review_reads_the_typed_amount() {
    assert!(reads_typed(SEND, 0, SEND, REVIEW));
    assert!(reads_typed(REVIEW, SEND, SEND, REVIEW));
    assert!(!reads_typed(REVIEW, DEPOSIT, SEND, REVIEW));
    assert!(!reads_typed(REVIEW, WITHDRAW, SEND, REVIEW));
    /* A send quoted before: the screen acted on now is a deposit. */
    assert!(!reads_typed(DEPOSIT, SEND, SEND, REVIEW));
    assert!(!reads_typed(WITHDRAW, SEND, SEND, REVIEW));
}

#[test]
fn a_pasted_address_loses_its_scheme_and_its_spaces() {
    assert_eq!(pasted_address(PINNED), Some(PINNED));
    let schemed = alloc::format!("nox:{PINNED}");
    assert_eq!(pasted_address(&schemed), Some(PINNED));
    let linked = alloc::format!("  NOX://{PINNED}\n");
    assert_eq!(pasted_address(&linked), Some(PINNED));
    assert_eq!(pasted_address("12.5"), None);
    assert_eq!(pasted_address("nox:"), None);
    assert_eq!(pasted_address(""), None);
}

#[test]
fn the_tail_shown_is_the_longest_that_fits() {
    let ten = |t: &str| 10 * t.chars().count() as i32;
    assert_eq!(tail_start("abc", 95, ten), 0);
    assert_eq!(tail_start("", 0, ten), 0);
    let text = "abcdefghijklmnop";
    let at = tail_start(text, 95, ten);
    assert_eq!(&text[at..], "hijklmnop");
    /* Every start is a character boundary. */
    let wide = "\u{e9}\u{e9}\u{e9}\u{e9}";
    let at = tail_start(wide, 25, ten);
    assert_eq!(&wide[at..], "\u{e9}\u{e9}");
    /* The address the field holds most: about a dozen measures, not 2,009. */
    let asked = core::cell::Cell::new(0u32);
    let at = tail_start(PINNED, 400, |t| {
        asked.set(asked.get() + 1);
        ten(t)
    });
    assert_eq!(PINNED.len() - at, 40);
    assert!(asked.get() <= 13, "{} measures", asked.get());
}

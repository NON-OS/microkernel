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

//! The clipboard takes an address and nothing else, and the wallet, the
//! keyring and the shield reach it only through the wallet's one gate.

use crate::wallet::copy_rule::copyable;

#[path = "copy_scan_tests.rs"]
mod scan;

const PUBLIC: &str = "0x871bc3AD5DA20c399d631817637cB5FB29eB04B4";

fn private() -> String {
    format!("nox1{}", "a".repeat(2005))
}

#[test]
fn a_whole_address_is_copied() {
    assert!(copyable(PUBLIC));
    assert!(copyable(&PUBLIC.to_ascii_lowercase()));
    assert!(copyable(&private()));
}

#[test]
fn a_key_or_words_or_a_hash_is_not() {
    let key = format!("0x{}", "1f".repeat(32));
    assert!(!copyable(&key), "a 64-digit private key");
    assert!(!copyable(&key[2..]), "the same key without 0x");
    assert!(!copyable("abandon ability able about above absent absorb abstract"));
    assert!(!copyable(""));
}

#[test]
fn a_near_address_is_not() {
    assert!(!copyable(&PUBLIC[..41]), "one digit short");
    assert!(!copyable(&format!("{PUBLIC}0")), "one digit long");
    assert!(!copyable(&format!(" {PUBLIC}")), "a space before");
    assert!(!copyable(&PUBLIC.replacen("0x", "0X", 1)), "not the 0x prefix");
    assert!(!copyable(&PUBLIC.replacen('8', "g", 1)), "a letter past f");
    assert!(!copyable(&private()[..2008]), "a cut private address");
    assert!(!copyable(&private().replacen('a', "1", 1)), "a digit base32 lacks");
}

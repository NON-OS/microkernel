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

//! A paste into Send is one whole address or nothing.

use crate::address_text::{pasted, NOT_AN_ADDRESS};

#[test]
fn a_paste_is_one_whole_address_or_nothing() {
    assert!(pasted("  0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed\n").is_ok());
    assert!(pasted("5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed").is_ok());
    let hash64 = "0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b";
    for bad in [
        hash64,
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAe",
        "Send 0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
        "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAeg",
        "",
    ] {
        assert_eq!(pasted(bad), Err(NOT_AN_ADDRESS), "{bad:?} was taken");
    }
}

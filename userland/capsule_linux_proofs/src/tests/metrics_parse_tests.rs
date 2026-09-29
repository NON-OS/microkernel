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

//! What a guest may say through /dev/nonos-metrics: listed names and
//! integers, and nothing that could carry text.

use crate::metrics_parse::{parse, NAMES};

#[test]
fn a_listed_name_and_an_integer_parse_to_the_same_name_and_number() {
    assert_eq!(parse(b"tokens=1"), Some(("tokens", 1)));
    assert_eq!(parse(b"ttft_ms=580"), Some(("ttft_ms", 580)));
    assert_eq!(parse(b"match=0"), Some(("match", 0)));
    for n in NAMES {
        let pair = alloc::format!("{n}=7");
        assert_eq!(parse(pair.as_bytes()), Some((n, 7)));
    }
}

#[test]
fn anything_that_could_carry_text_is_refused() {
    for bad in [
        &b"text=Paris"[..],
        b"Paris",
        b"tokens=",
        b"tokens=1.5",
        b"tokens=-1",
        b"tokens=0x10",
        b"tokens=1e3",
        b"TOKENS=1",
        b"tokens =1",
        b"=1",
        b"tokens=12345678901234567",
        b"tokens=1000000000000001",
        b"prompt=What",
    ] {
        assert_eq!(parse(bad), None, "{:?}", core::str::from_utf8(bad));
    }
}

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

//! The secp256r1 agreement against RFC 5903 section 8.1, both directions.

use crate::p256_share::{generate, shared};

fn hex<const N: usize>(s: &str) -> [u8; N] {
    let mut out = [0u8; N];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[2 * i..2 * i + 2], 16).expect("hex");
    }
    out
}

fn point(x: &str, y: &str) -> [u8; 65] {
    hex(&format!("04{x}{y}"))
}

const I: &str = "C88F01F510D9AC3F70A292DAA2316DE544E9AAB8AFE84049C62A9C57862D1433";
const R: &str = "C6EF9C5D78AE012A011164ACB397CE2088685D8F06BF9BE0B283AB46476BEE53";
const GIX: &str = "DAD0B65394221CF9B051E1FECA5787D098DFE637FC90B9EF945D0C3772581180";
const GIY: &str = "5271A0461CDB8252D61F1C456FA3E59AB1F45B33ACCF5F58389E0577B8990BB3";
const GRX: &str = "D12DFB5289C8D4F81208B70270398C342296970A0BCCB74C736FC7554494BF63";
const GRY: &str = "56FBF3CA366CC23E8157854C13C58D6AAC23F046ADA30F8353E74F33039872AB";
const GIRX: &str = "D6840F6B42F6EDAFD13116E0E12565202FEF8E9ECE7DCE03812464D04B9442DE";

#[test]
fn the_shared_secret_is_the_rfc_5903_x_coordinate_from_either_side() {
    assert_eq!(shared(&hex(I), &point(GRX, GRY)), Some(hex::<32>(GIRX)));
    assert_eq!(shared(&hex(R), &point(GIX, GIY)), Some(hex::<32>(GIRX)));
}

#[test]
fn a_point_off_the_curve_is_refused() {
    let mut bad = point(GRX, GRY);
    bad[64] ^= 1;
    assert_eq!(shared(&hex(I), &bad), None);
}

#[test]
fn a_fresh_share_is_an_uncompressed_point_that_agrees_with_itself() {
    let (a_private, a_public) = generate().expect("a share");
    let (b_private, b_public) = generate().expect("a second share");
    assert_eq!(a_public[0], 4);
    assert_ne!(a_public, b_public, "two draws gave one key");
    let ab = shared(&a_private, &b_public).expect("a with b");
    assert_eq!(Some(ab), shared(&b_private, &a_public), "the agreement is symmetric");
}

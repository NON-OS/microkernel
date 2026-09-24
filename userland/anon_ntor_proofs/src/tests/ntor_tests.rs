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

//! The two ntor strings, against the reference field order.

extern crate alloc;

use alloc::string::String;

use crate::constants::{AUTH_INPUT_BYTES, KEY_MATERIAL_BYTES, SECRET_INPUT_BYTES};
use crate::hex;
use crate::inputs::{auth_input, secret_input, Parts};
use crate::ntor_vector::{B, NODE_ID, PROTOID_HEX, SERVER_HEX, VERIFY, X, XB, XY, Y};

fn fixed<const N: usize>(text: &str) -> [u8; N] {
    let bytes = hex(text);
    let mut out = [0u8; N];
    out.copy_from_slice(&bytes);
    out
}

#[test]
fn widths_match_the_fork() {
    /*
     * From onion_ntor.c: SECRET_INPUT_LEN and AUTH_INPUT_LEN, and from or.h:
     * CPATH_KEY_MATERIAL_LEN.
     */
    assert_eq!(SECRET_INPUT_BYTES, 204);
    assert_eq!(AUTH_INPUT_BYTES, 178);
    assert_eq!(KEY_MATERIAL_BYTES, 72);
}

#[test]
fn secret_input_matches_reference() {
    let (xy, xb, id, b, x, y) =
        (fixed(XY), fixed(XB), fixed(NODE_ID), fixed(B), fixed(X), fixed(Y));
    let built = secret_input(&Parts {
        xy: &xy,
        xb: &xb,
        identity: &id,
        onion_key: &b,
        client: &x,
        server: &y,
    });
    let want: String = [XY, XB, NODE_ID, B, X, Y, PROTOID_HEX].concat();
    assert_eq!(built.as_slice(), hex(&want).as_slice());
}

#[test]
fn auth_input_matches_reference() {
    let (xy, xb, id, b, x, y) =
        (fixed(XY), fixed(XB), fixed(NODE_ID), fixed(B), fixed(X), fixed(Y));
    let built = auth_input(
        &Parts { xy: &xy, xb: &xb, identity: &id, onion_key: &b, client: &x, server: &y },
        &fixed(VERIFY),
    );
    let want: String = [VERIFY, NODE_ID, B, Y, X, PROTOID_HEX, SERVER_HEX].concat();
    assert_eq!(built.as_slice(), hex(&want).as_slice());
}

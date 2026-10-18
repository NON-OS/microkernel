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

//! Hostile ops: one not served, and a served one with a body of another
//! length. Each has its named status, as virtio-gpu names it, and moves no
//! pixel. A body too long is one the op takes with bytes after it, so only
//! its length refuses it: a body of zeros names resource 0 or an empty
//! rectangle, which is refused whatever its length.

use super::frames::{ask, flush_body, scanout_body, transfer_body};
use super::model::{paint, served, take, COMPOSITOR};
use crate::constants::CLEAR_COLOR;
use crate::serve::OP_TRANSFER_TO_HOST;
use crate::serve::{E_BAD_OP, E_INVAL, OP_FLUSH, OP_GET_PRIMARY_SURFACE, OP_SET_SCANOUT};

#[test]
fn an_op_not_served_is_e_bad_op_and_e_inval_with_a_body() {
    let mut s = served(64, 48);
    for op in [0, 1, 2, 3, 4, 5, 6, 7, 0x0B, 0x0D, 0x0100, 0xFFFF] {
        assert_eq!(ask(&mut s.d, COMPOSITOR, op, &[]).0, E_BAD_OP, "op {op:#x}");
        assert_eq!(ask(&mut s.d, COMPOSITOR, op, &[0; 4]).0, E_INVAL, "op {op:#x}");
    }
    assert_eq!(ask(&mut s.d, COMPOSITOR, OP_GET_PRIMARY_SURFACE, &[0; 4]).0, E_INVAL);
}

#[test]
fn a_served_op_with_a_body_of_another_length_is_refused_and_moves_nothing() {
    let mut s = served(64, 48);
    take(&mut s);
    paint(&s);
    let taken = [
        (OP_TRANSFER_TO_HOST, transfer_body(0, 0, 1, 1, 0)),
        (OP_SET_SCANOUT, scanout_body(0, 0, 64, 48)),
        (OP_FLUSH, flush_body(0, 0, 1, 1)),
    ];
    for (op, body) in &taken {
        let len = body.len();
        for short in [0, 1, len - 1] {
            assert_eq!(ask(&mut s.d, COMPOSITOR, *op, &body[..short]).0, E_INVAL, "{op} {short}");
        }
        for long in [len + 1, 64] {
            let body = [&body[..], &vec![0; long - len]].concat();
            assert_eq!(ask(&mut s.d, COMPOSITOR, *op, &body).0, E_INVAL, "{op} {long}");
        }
    }
    assert!((0..64 * 48).all(|i| s.a.fb.wrote32(i * 4) == CLEAR_COLOR), "a pixel moved");
    for (op, body) in taken {
        assert_eq!(ask(&mut s.d, COMPOSITOR, op, &body).0, 0, "{op}: the body itself refused");
    }
}

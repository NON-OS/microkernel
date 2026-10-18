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

//! The request header and body held to what they claim: a header that
//! does not decode, a body longer or shorter than it says, a body where
//! the op takes none, and an op the service does not have.

use super::rig::{sdhc, serving};
use super::service_rig::{answer_raw, call, request, span, KERNEL};
use crate::protocol::*;

#[test]
fn a_header_that_does_not_decode_is_answered_einval_under_an_empty_header() {
    let (mut drv, mut medium) = serving(sdhc(1));
    let good = request(OP_CAPACITY, 3, &[]);
    let mut magic = good.clone();
    magic[0] ^= 1;
    let mut version = good.clone();
    version[4] = 2;
    for bad in [vec![], good[..19].to_vec(), magic, version] {
        let r = answer_raw(&mut drv, &mut medium, KERNEL, &bad);
        assert_eq!((r.status, r.op, r.id, r.payload.len()), (E_INVAL, 0, 0, 0));
    }
}

#[test]
fn a_body_is_held_to_the_length_the_header_claims_and_the_op_allows() {
    let (mut drv, mut medium) = serving(sdhc(1));
    let mut short = request(OP_READ, 4, &span(0, 1));
    short.pop();
    let mut long = request(OP_CAPACITY, 4, &[]);
    long.extend([1, 2, 3, 4]);
    for rx in [short, long] {
        assert_eq!(answer_raw(&mut drv, &mut medium, KERNEL, &rx).status, E_MSGSIZE);
    }
    for op in [OP_HEALTHCHECK, OP_CAPACITY, OP_FLUSH, OP_IDENTIFY] {
        assert_eq!(call(&mut drv, &mut medium, KERNEL, op, &[0]).status, E_INVAL);
    }
    for op in [0, 7, 0xFFFF] {
        assert_eq!(call(&mut drv, &mut medium, KERNEL, op, &[]).status, E_INVAL);
    }
}

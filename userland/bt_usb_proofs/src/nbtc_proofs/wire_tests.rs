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

//! NBTC's frame: the header NNET uses, refused whole when any field of it
//! is wrong, with the request id still named in the refusal.

use super::fixture::{board, raw, reply, request, ID};
use crate::nbtc::ops::{E_BUSY, E_INVAL, OP_HEALTH, OP_LINK};
use crate::nbtc::who::Caller::Settings;
use crate::nbtc::wire::{decode, REPLY_MAX};
use crate::nbtc::{busy, Board};
use nonos_bt_bond::{MemoryKeys, Store};

#[test]
fn a_request_decodes_to_its_op_id_and_payload() {
    let f = request(OP_LINK, &[7, 8]);
    let (req, body) = decode(&f).unwrap();
    assert_eq!((req.op, req.request_id, body), (OP_LINK, ID, &[7u8, 8][..]));
}

#[test]
fn each_header_field_is_checked() {
    let good = request(OP_HEALTH, &[]);
    // Magic, version, the zero half-word, and a length that disagrees.
    for (at, v) in [(0, b'X'), (4, 2), (10, 1), (16, 1)] {
        let mut f = good.clone();
        f[at] = v;
        let (status, data) = raw(&mut board(), Settings, &f, 0, OP_HEALTH);
        assert_eq!((status, data.len()), (E_INVAL, 0), "field at {at}");
    }
    let mut short = good.clone();
    short.push(0);
    assert!(decode(&short).is_err(), "a trailing octet the length does not count");
}

#[test]
fn a_frame_too_short_for_a_header_is_refused_under_id_zero() {
    let mut b = board();
    let mut tx = [0u8; REPLY_MAX];
    let n = crate::nbtc::answer(&mut b, Settings, &[0x4e, 0x42], 0, &mut tx);
    assert_eq!(&tx[12..16], &[0; 4]);
    assert_eq!(i32::from_le_bytes(tx[20..24].try_into().unwrap()), E_INVAL);
    assert_eq!(n, 24);
}

#[test]
fn a_busy_board_refuses_with_the_request_named() {
    let mut tx = [0u8; REPLY_MAX];
    let n = busy(&request(OP_LINK, &[]), &mut tx);
    assert_eq!(reply(&tx[..n], OP_LINK), (E_BUSY, vec![]));
}

#[test]
fn ops_that_take_nothing_refuse_a_payload() {
    let mut b: Board<MemoryKeys> = Board::new(Store::open(MemoryKeys::default()).unwrap());
    let f = request(OP_HEALTH, &[0]);
    assert_eq!(raw(&mut b, Settings, &f, 0, OP_HEALTH), (E_INVAL, vec![]));
}

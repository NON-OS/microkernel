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

//! The session's ending order, EXIT then RECORD then CLOSE, and the
//! record's cap.

use super::aborted_with;
use super::check::{gets, logged_in};
use super::own::{session_record, Other};
use crate::kit::pair::Pair;
use nsh_client::Refusal;
use nsh_wire::{Kind, Reason, WireError};

fn part(last: bool, bytes: &[u8]) -> Vec<u8> {
    [&[u8::from(last)][..], bytes].concat()
}

fn after(steps: Vec<(Kind, Vec<u8>)>, then: (Kind, Vec<u8>)) -> (Result<(), Refusal>, Pair) {
    let mut p = logged_in();
    steps.iter().for_each(|(k, b)| gets(&mut p, *k, b).unwrap());
    (gets(&mut p, then.0, &then.1), p)
}

#[test]
fn the_ending_order_is_kept() {
    let exit = (Kind::Exit, 0i32.to_le_bytes().to_vec());
    let rec = session_record(logged_in().server.th_hello, Other::Nothing);
    let open_part = (Kind::Record, part(false, &rec[..100]));
    let last_part = (Kind::Record, part(true, &rec));
    let out = (Kind::Output, b"x".to_vec());
    let close = (Kind::Close, vec![]);
    let order = Err(Refusal::Wire(WireError::Order));
    assert_eq!(after(vec![exit.clone()], out.clone()).0, order);
    assert_eq!(after(vec![exit.clone()], exit.clone()).0, order);
    assert_eq!(after(vec![open_part.clone()], out.clone()).0, order);
    assert_eq!(after(vec![open_part.clone()], close.clone()).0, order, "no CLOSE inside a record");
    assert_eq!(after(vec![open_part.clone()], exit.clone()).0, order);
    assert_eq!(after(vec![last_part.clone()], last_part.clone()).0, order);
    assert_eq!(after(vec![last_part.clone()], out).0, order);
    assert_eq!(after(vec![exit.clone()], close.clone()).0, order, "no CLOSE before the record");
    assert_eq!(after(vec![], close.clone()).0, order);
    let (r, mut p) = after(vec![exit, last_part], close);
    assert_eq!(r, Ok(()));
    assert!(p.client.is_over());
    let (r, mut q) = after(vec![], (Kind::Record, vec![2, 1]));
    assert_eq!(r, Err(Refusal::Wire(WireError::Value)));
    aborted_with(&mut q, Reason::Protocol);
    p.client.take_output();
}

#[test]
fn a_record_past_its_cap_is_refused_with_limit() {
    let mut p = logged_in();
    let chunk = vec![0x5A; 16_383];
    let parts = (64usize << 20) / chunk.len();
    (0..parts).for_each(|_| gets(&mut p, Kind::Record, &part(false, &chunk)).unwrap());
    let r = gets(&mut p, Kind::Record, &part(true, &chunk));
    assert_eq!(r, Err(Refusal::RecordLimit));
    aborted_with(&mut p, Reason::Limit);
}

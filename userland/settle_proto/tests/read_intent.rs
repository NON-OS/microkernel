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

use nonos_settle_proto::*;

fn cuts_refused<T: core::fmt::Debug>(b: &[u8], parse: fn(&[u8]) -> Option<T>) {
    for n in 0..b.len() {
        assert!(parse(&b[..n]).is_none(), "{n} of {} bytes", b.len());
    }
    let mut long = b.to_vec();
    long.push(0);
    assert!(parse(&long).is_none(), "a trailing byte");
}

#[test]
fn read_and_its_reply_round_trip() {
    let r = Read { chain_id: 11_155_111, view: 60, args: vec![[1; 32], [2; 32]] };
    let b = r.encode().unwrap();
    assert_eq!((&b[8..10], b[10], b.len()), (&[60u8, 0][..], 2, 75));
    assert_eq!(Read::parse(&b), Some(r));
    cuts_refused(&b, Read::parse);
    let too_many = Read { chain_id: 1, view: 1, args: vec![[0; 32]; 256] };
    assert_eq!(too_many.encode(), None);
    let ret = read_reply(&[[3; 32], [4; 32], [5; 32]]).unwrap();
    assert_eq!(parse_read_reply(&ret), Some(vec![[3; 32], [4; 32], [5; 32]]));
    cuts_refused(&ret, parse_read_reply);
    assert_eq!(read_reply(&[[0; 32]; 256]), None);
}

#[test]
fn intent_round_trips_and_its_calldata_is_bounded() {
    let mut call = 0x39c79e0cu32.to_be_bytes().to_vec();
    call.extend_from_slice(&[8; 32]);
    let i = Intent { chain_id: 1, kind: 15, from: [6; 20], value: 0, calldata: call };
    let b = i.encode().unwrap();
    assert_eq!((b[8], b.len()), (15, 49 + 36));
    assert_eq!(Intent::parse(&b), Some(i.clone()));
    cuts_refused(&b, Intent::parse);
    let mut big = b[..45].to_vec();
    big.extend_from_slice(&((MAX_CALLDATA + 1) as u32).to_le_bytes());
    big.resize(49 + MAX_CALLDATA + 1, 0);
    assert_eq!(Intent::parse(&big), None, "past MAX_CALLDATA");
    let at = Intent { calldata: vec![0; MAX_CALLDATA], ..i.clone() };
    assert_eq!(at.encode().map(|b| b.len()), Some(49 + MAX_CALLDATA));
    let past = Intent { calldata: vec![0; MAX_CALLDATA + 1], ..i };
    assert_eq!(past.encode(), None, "one past MAX_CALLDATA is refused, not written");
}

#[test]
fn a_quote_round_trips_and_pads_by_a_fifth() {
    let q =
        Quote { kind: 13, to: [1; 20], nonce: 7, gas_limit: 240_000, max_fee: 9, max_priority: 2 };
    let b = q.encode();
    assert_eq!(Quote::parse(&b), Some(q));
    cuts_refused(&b, Quote::parse);
    assert_eq!((padded(200_000), padded(u64::MAX)), (Some(240_000), None));
}

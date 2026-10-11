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

use super::fixture::{funded_state, LISTING, NOW, PAYEE, ROOT};
use nonos_nox_meter::wire::{account_reply, drain_reply, parse_budget, Identity};
use nonos_nox_meter::wire::{ACCOUNT_ROW_LEN, KEPT, READY, VOUCHER_RECORD_LEN};
use nonos_nox_meter::MeterError;

#[test]
fn a_budget_body_parses_exactly_or_not_at_all() {
    let mut b = 500u128.to_le_bytes().to_vec();
    b.extend_from_slice(&(NOW + 9).to_le_bytes());
    b.push(1);
    b.extend_from_slice(&LISTING);
    b.extend_from_slice(&40u128.to_le_bytes());
    let got = parse_budget(&b).unwrap();
    assert_eq!((got.cap, got.expiry, got.listings[0].cap), (500, NOW + 9, 40));
    for n in 0..b.len() {
        assert_eq!(parse_budget(&b[..n]), Err(MeterError::Corrupt), "{n} bytes");
    }
    b.push(0);
    assert_eq!(parse_budget(&b), Err(MeterError::Corrupt));
}

#[test]
fn account_shows_identity_budget_and_channel_rows() {
    let s = funded_state();
    let none = account_reply(&s, None, false, NOW, 0);
    assert_eq!(none.len(), 111 + ACCOUNT_ROW_LEN);
    assert!(none[..53].iter().all(|&x| x == 0));
    let who = Identity { address: [0xaa; 20], ed25519: [0xed; 32] };
    let r = account_reply(&s, Some(&who), true, NOW, 0);
    assert_eq!((r[0], &r[1..21], &r[21..53]), (READY | KEPT, &[0xaa; 20][..], &[0xed; 32][..]));
    assert_eq!(&r[93..109], &500u128.to_le_bytes());
    assert_eq!((r[109], r[110], &r[111..143]), (1, 1, &s.channels[0].id[..]));
    assert_eq!(account_reply(&s, None, true, NOW, 1)[..1], [KEPT]);
    assert_eq!(account_reply(&s, None, false, NOW, 1)[109..111], [1, 0]);
}

#[test]
fn drain_hands_out_only_signed_vouchers() {
    let payer = [0xaa; 20];
    let (s, c) = funded_state().charged(&PAYEE, &LISTING, 1, 7, ROOT, NOW).unwrap();
    assert_eq!(drain_reply(&s, &payer, 0), vec![0, 0]);
    let s = s.signed(&c.id, c.seq, [9; 65]);
    let r = drain_reply(&s, &payer, 0);
    assert_eq!(r.len(), 2 + VOUCHER_RECORD_LEN);
    assert_eq!((r[0], r[1], &r[2..34], &r[34..54]), (1, 1, &c.id[..], &payer[..]));
    assert_eq!(&r[138..154], &7u128.to_le_bytes());
    assert_eq!(&r[2 + VOUCHER_RECORD_LEN - 65..], &[9; 65]);
}

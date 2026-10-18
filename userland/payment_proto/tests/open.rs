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

//! OPEN's two forms.

use nonos_payment_proto::*;

fn start() -> OpenStart {
    OpenStart {
        chain_id: 1,
        payee: [2; 20],
        listing_id: [3; 32],
        unit: [4; 32],
        amount: 5,
        expiry: 6,
        reclaim_to: [7; 20],
    }
}

#[test]
fn start_and_confirm_are_told_apart_by_their_first_byte() {
    let s = start().encode();
    assert_eq!((s[0], s.len()), (OPEN_START, 1 + 8 + 20 + 32 + 32 + 16 + 8 + 20));
    assert_eq!(parse_open(&s), Some(Ok(start())));
    let c = OpenConfirm { start: start(), open_tx: [8; 32] };
    let b = c.encode();
    assert_eq!((b[0], b.len()), (OPEN_CONFIRM, s.len() + 32));
    assert_eq!(parse_open(&b), Some(Err(c)));
    for n in 0..b.len() {
        if n != s.len() {
            assert_eq!(parse_open(&b[..n]), None, "{n} bytes");
        }
    }
    let mut other = s.clone();
    other[0] = 2;
    assert_eq!(parse_open(&other), None);
}

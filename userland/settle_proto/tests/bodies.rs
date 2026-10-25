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

#[test]
fn channel_state_gas_and_opened_round_trip_exactly() {
    let s = ChannelState { deposited: 1 << 100, settled: 5, expiry: 1_900_000_000, closed: true };
    let b = s.encode();
    assert_eq!((ChannelState::parse(&b), b[40]), (Some(s), 1));
    let mut bad = b;
    bad[40] = 2;
    assert_eq!(ChannelState::parse(&bad), None);
    assert_eq!(ChannelState::parse(&b[..40]), None);
    let g = Gas { balance: 3, tx_nonce: 4, max_fee_per_gas: 5, max_priority_fee_per_gas: 6 };
    assert_eq!(Gas::parse(&g.encode()), Some(g));
    assert_eq!(Gas::parse(&g.encode()[1..]), None);
    let req = Gas::request(1, &[0xaa; 20]);
    assert_eq!((&req[..8], &req[8..]), (&1u64.to_le_bytes()[..], &[0xaa; 20][..]));
    let o = Opened {
        channel_id: [1; 32],
        payer: [2; 20],
        payee: [3; 20],
        listing_id: [4; 32],
        amount: 5,
        expiry: 6,
        open_nonce: 7,
        reclaim_to: [8; 20],
    };
    assert_eq!(Opened::parse(&o.encode()), Some(o));
    assert_eq!(Opened::parse(&o.encode()[..OPENED_LEN - 1]), None);
    let ch = chain_hash(11_155_111, &[9; 32]);
    assert_eq!(parse_chain_hash(&ch), Some(ChainHash { chain_id: 11_155_111, id: [9; 32] }));
    assert_eq!(parse_chain_hash(&ch[1..]), None);
}

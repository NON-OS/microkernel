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
fn a_channel_state_round_trips_and_nothing_else_parses() {
    let s = ChannelState { deposited: 1 << 100, settled: 5, expiry: 1_900_000_000, closed: true };
    let b = channel_state_reply(&s);
    assert_eq!(parse_channel_state(&b), Some(s));
    assert_eq!(&b[..16], &(1u128 << 100).to_le_bytes());
    assert_eq!(b[40], 1);
    for n in 0..b.len() {
        assert_eq!(parse_channel_state(&b[..n]), None);
    }
    let mut bad = b;
    bad[40] = 2;
    assert_eq!(parse_channel_state(&bad), None);
    let r = channel_state_request(11_155_111, &[7; 32]);
    assert_eq!((&r[..8], &r[8..]), (&11_155_111u64.to_le_bytes()[..], &[7u8; 32][..]));
}

#[test]
fn the_ops_and_service_name_are_pinned() {
    assert_eq!(SERVICE, b"nox_settle");
    let ops =
        [OP_HEALTHCHECK, OP_BROADCAST, OP_READ_NONCE, OP_CHANNEL_STATE, OP_ETH_BALANCE, OP_OPENED];
    assert_eq!(ops, [1, 2, 3, 4, 5, 6]);
}

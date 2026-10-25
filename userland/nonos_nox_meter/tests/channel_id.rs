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

//! The channel id against its encoding built here by hand from plan/21.

mod host;

use host::{keccak, Host};
use nonos_nox_meter::voucher::ChannelKey;

#[test]
fn the_channel_id_follows_plan_21() {
    let key = ChannelKey {
        chain_id: 1,
        settlement: [0x5e; 20],
        payer: [0xaa; 20],
        payee: [0xbb; 20],
        listing_id: [2; 32],
        open_nonce: 3,
    };
    let word = |n: u64| [[0u8; 24].as_slice(), &n.to_be_bytes()].concat();
    let addr = |a: &[u8; 20]| [[0u8; 12].as_slice(), a].concat();
    let mut pre = keccak(b"NONOS.nox.channel.v1").to_vec();
    pre.extend_from_slice(&word(1));
    pre.extend_from_slice(&addr(&key.settlement));
    pre.extend_from_slice(&addr(&key.payer));
    pre.extend_from_slice(&addr(&key.payee));
    pre.extend_from_slice(&key.listing_id);
    pre.extend_from_slice(&word(3));
    assert_eq!(key.id(&Host), keccak(&pre));
    // A reopened channel, another chain or another settlement is another id.
    for other in [
        ChannelKey { open_nonce: 4, ..key },
        ChannelKey { chain_id: 11_155_111, ..key },
        ChannelKey { settlement: [0x5f; 20], ..key },
    ] {
        assert_ne!(other.id(&Host), key.id(&Host));
    }
}

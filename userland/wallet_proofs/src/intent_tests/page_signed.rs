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

/* The fields signed carry the very calldata the page was read from, the
 * quote gives only `to`, the nonce and the gas terms, and nothing is built
 * for a network whose contracts are not deployed. */

use nonos_nox_config::SEPOLIA;

use super::decode::built;
use super::fixture::quote;
use super::kinds::every;
use super::page::page;
use crate::intent::{gwei, signable, PublisherSend};

#[test]
fn what_is_signed_is_what_was_read() {
    for p in every() {
        let send =
            PublisherSend { kind: p.kind(), chain_id: 1, from: [9; 20], calldata: built(&p) };
        let q = quote(p.kind());
        let s = signable(&send, &q);
        assert_eq!((s.data.as_slice(), s.to, s.value), (send.calldata.as_slice(), q.to, 0));
        assert_eq!(
            (s.nonce, s.gas, s.max_fee, s.max_priority),
            (4, 60_000, 2_000_000_000, 100_000_000)
        );
        assert_eq!(page(p.kind(), &s.data), page(p.kind(), &send.calldata));
        assert_eq!(PublisherSend::new(&p, SEPOLIA, [9; 20]), None, "not on Sepolia yet");
    }
    assert_eq!((gwei(2_000_000_000), gwei(100_000_000)), ("2 gwei".into(), "0.1 gwei".into()));
    assert_eq!((gwei(1), gwei(0)), ("0.000000001 gwei".into(), "0 gwei".into()));
}

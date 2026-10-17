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

/* A publisher transaction's first steps: no start while a wallet send is
 * pending (Q8), and the quote held to the calldata's own decode and to the
 * wallet's bounds. */

use nonos_settle_proto::Quote;

use super::decode::built;
use super::fixture::{at, quote};
use super::kinds::every;
use crate::intent::{after_quote, start, IntentStep, PublisherSend, NOT_OURS, SEND_PENDING};

fn send(kind: u8, calldata: Vec<u8>) -> PublisherSend {
    PublisherSend { kind, chain_id: 1, from: [9; 20], calldata }
}

#[test]
fn no_start_while_a_wallet_send_is_pending() {
    assert_eq!(start(true), IntentStep::Failed(SEND_PENDING));
    assert_eq!(start(false), IntentStep::Quoting);
}

#[test]
fn the_quote_is_held_to_the_calldata_and_the_bounds() {
    for p in every() {
        let (s, q) = (send(p.kind(), built(&p)), quote(p.kind()));
        assert_eq!(after_quote(&s, &q, &at), IntentStep::Funds(q), "{p:?}");
        let moved = Quote { to: [0x7E; 20], ..q };
        assert!(matches!(after_quote(&s, &moved, &at), IntentStep::Failed(_)), "{p:?}");
        let greedy = Quote { gas_limit: 1_000_001, ..q };
        assert!(matches!(after_quote(&s, &greedy, &at), IntentStep::Failed(_)), "{p:?}");
        let mut cd = built(&p);
        cd[0] ^= 1;
        assert_eq!(after_quote(&send(p.kind(), cd), &q, &at), IntentStep::Failed(NOT_OURS));
        let undeployed = after_quote(&s, &q, &|_| None);
        assert!(matches!(undeployed, IntentStep::Failed(_)), "not on this network {p:?}");
    }
}

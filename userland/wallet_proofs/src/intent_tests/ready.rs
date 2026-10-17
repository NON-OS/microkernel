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

/* At Sign, intent::ready's last checks: the network the review was on,
 * calldata that reads back, the reviewed quote's nonce still the chain's
 * pending one (Q8), the quote held; then exactly the send's own calldata
 * signed (D5), never anything built after the review. */

use nonos_settle_proto::{Quote, KINDS};

use super::decode::built;
use super::fixture::{at, quote};
use super::kinds::every;
use crate::intent::{held, ready, signable, IntentStep, Publisher, PublisherSend};
use crate::intent::{CHAIN_MOVED, NONCE_MOVED, NOT_READ_BACK, NOT_REVIEWED};

fn send(p: &Publisher) -> PublisherSend {
    PublisherSend { kind: p.kind(), chain_id: 1, from: [9; 20], calldata: built(p) }
}

/// The quote's bounds, `to` the fixture's contract for the kind.
fn bounds(p: &Publisher, q: &Quote) -> Option<&'static str> {
    held(p, KINDS.iter().find(|k| k.id == p.kind()).and_then(|k| at(k.contract)), q)
}

#[test]
fn signs_the_reviewed_calldata_itself_at_the_quoted_nonce() {
    for p in every() {
        let (s, q) = (send(&p), quote(p.kind()));
        let t = ready(&s, &IntentStep::Review { quote: q }, 1, q.nonce, &at, bounds).unwrap();
        assert_eq!(t, signable(&s, &q), "{p:?}");
        assert!(!t.data.is_empty() && t.data == s.calldata, "{p:?}: the send's own bytes");
        assert_eq!((t.to, t.value, t.nonce), (q.to, 0, q.nonce));
    }
}

#[test]
fn nothing_is_signed_once_the_nonce_moved_or_nothing_was_reviewed() {
    for p in every() {
        let (s, q) = (send(&p), quote(p.kind()));
        let review = IntentStep::Review { quote: q };
        for pending in [q.nonce + 1, q.nonce - 1, 0, u64::MAX] {
            assert_eq!(ready(&s, &review, 1, pending, &at, bounds), Err(NONCE_MOVED));
        }
        let funds = IntentStep::Funds(q);
        assert_eq!(ready(&s, &funds, 1, q.nonce, &at, bounds), Err(NOT_REVIEWED));
    }
}

#[test]
fn another_network_bytes_not_ours_or_a_quote_out_of_bounds_are_refused() {
    let p = every().remove(0);
    let (s, q) = (send(&p), quote(p.kind()));
    let review = IntentStep::Review { quote: q };
    assert_eq!(ready(&s, &review, 11_155_111, q.nonce, &at, bounds), Err(CHAIN_MOVED));
    let mut longer = s.clone();
    longer.calldata.push(0);
    assert_eq!(ready(&longer, &review, 1, q.nonce, &at, bounds), Err(NOT_READ_BACK));
    let elsewhere = IntentStep::Review { quote: Quote { to: [1; 20], ..q } };
    let why = ready(&s, &elsewhere, 1, q.nonce, &at, bounds);
    assert_eq!(why, Err("settle quoted a contract other than the config's, not signing"));
}

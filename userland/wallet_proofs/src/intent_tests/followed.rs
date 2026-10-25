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

/* The publisher screen says what the send ledger holds for the hash signed:
 * sent while it may land, never sent or landed once replaced or dropped. */

use nonos_settle_proto::Quote;

use super::fixture::quote;
use super::ledger::send;
use crate::hex_id::id;
use crate::intent::followed::{fate_of, may_land, said};
use crate::intent::retry::after_sign;
use crate::intent::unsure::after_broadcast;
use crate::intent::IntentStep;
use crate::sent::{Fate, Ledger, Reading, CONFIRMATIONS, DROP_MS, REPLACED_AFTER};

pub(super) const OURS: [u8; 32] = [3; 32];
pub(super) const NODE: [u8; 32] = [4; 32];
type Read = Option<(Option<(u64, bool)>, u64, u64)>;

/// Signed as OURS, settle answering NODE; then, if `read`, read REPLACED_AFTER times
/// past DROP_MS: its receipt, the head and how far the account's nonce is past it.
pub(super) fn sent(read: Read) -> (Ledger, IntentStep, Quote) {
    let (s, q, mut l) = (send(), quote(send().kind), Ledger::default());
    after_sign(&mut l, &s, Ok((q, OURS)), 1);
    let step = after_broadcast(Some(NODE), Some((q, OURS)));
    let (receipt, head, used) = read.unwrap_or((None, 0, 0));
    let r = Reading { receipt, head, latest: q.nonce + used, pending: q.nonce + used };
    (0..REPLACED_AFTER * u8::from(read.is_some())).for_each(|_| _ = l.judge(&OURS, r, 2 + DROP_MS));
    (l, step, q)
}

pub(super) fn words(l: &Ledger, step: &IntentStep) -> String {
    let IntentStep::Sent(h) = step else { panic!("{step:?}") };
    said(h, fate_of(l, step), " 1 more follow.")
}

#[test]
fn each_fate_is_said_as_the_ledger_holds_it_and_only_a_live_send_as_sent() {
    let live = format!("Sent {}; waiting for it to land. 1 more follow.", id(&OURS));
    let (deep, ok, bad) = (10 + CONFIRMATIONS, Some((10, true)), Some((10, false)));
    let cases = [
        (None, Fate::Waiting, live.as_str(), "Its own hash, the one followed:"),
        (Some((ok, 10, 1)), Fate::InBlock { block: 10, ok: true }, &live, "followed"),
        (Some((ok, deep, 1)), Fate::Confirmed { ok: true }, &live, "followed"),
        (Some((bad, 10, 1)), Fate::InBlock { block: 10, ok: false }, "Reverted: ", "fee"),
        (Some((None, 10, 1)), Fate::Replaced, "Replaced: another", "did not take effect"),
        (Some((None, 10, 0)), Fate::Dropped, "Dropped: the network", "nothing was sent"),
    ];
    for (read, fate, starts, has) in cases {
        let (l, step, _) = sent(read);
        assert_eq!(fate_of(&l, &step), Some(fate), "the ledger's own fate, read each paint");
        let w = words(&l, &step);
        assert!(w.starts_with(starts) && w.contains(has), "{fate:?}: {w}");
        assert!(w.contains(&id(&OURS)) && !w.contains(&id(&NODE)), "{w}");
        assert_eq!(may_land(&l, &step), starts == live, "{fate:?}");
        if starts != live {
            assert!(!w.contains("Sent") && !w.contains("land") && !w.contains("more"), "{w}");
        }
    }
}

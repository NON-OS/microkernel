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

/*
 * After a Sign press (B8, the Lead's ruling on commit 4): no retry or
 * re-quote signs or broadcasts on its own; each Turn here and in
 * unsure.rs leaves no press standing, so only the user's next guarded
 * Sign signs. Signed, it is in the wallet's one send ledger before it
 * goes, counted by both paths. A moved nonce is quoted again, read-only.
 */

use nonos_settle_proto::Quote;

use super::quote::max_cost;
use super::send::PublisherSend;
use super::step::{IntentStep, NONCE_MOVED};
use crate::wallet::send::sent::{Fate, Ledger, Sent};

/// A step, and whether a Sign press stands for it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Turn {
    pub step: IntentStep,
    pub pressed: bool,
}

pub(super) fn to(step: IntentStep) -> Turn {
    Turn { step, pressed: false }
}

/// Sign's outcome: what was signed at a quote with its own hash, or why not.
pub fn after_sign(
    ledger: &mut Ledger,
    send: &PublisherSend,
    signed: Result<(Quote, [u8; 32]), &'static str>,
    now_ms: i64,
) -> Turn {
    to(match signed {
        Ok((q, hash)) => {
            ledger.record_publisher(Sent {
                chain: send.chain_id,
                from: send.from,
                nonce: q.nonce,
                hash,
                at_ms: now_ms,
                eth_cost: max_cost(&q),
                token: None,
                fate: Fate::Waiting,
                misses: 0,
                unknown: false,
            });
            IntentStep::Broadcasting
        }
        Err(why) if why == NONCE_MOVED => IntentStep::Quoting,
        Err(why) => IntentStep::Failed(why),
    })
}

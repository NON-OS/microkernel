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
 * A broadcast settle did not confirm (B8, the maintainers' ruling on commit 4):
 * it may not have gone, so it is read on chain before anything else. GAS's
 * pending nonce counts what waits (capsule_nox_settle handlers/gas.rs);
 * settle offers no read of a transaction by hash. Its nonce used, it is
 * followed as sent and never sent again. Unused, the same transaction at
 * the same nonce is offered again, behind the guarded Sign only.
 */

use nonos_settle_proto::Quote;

use super::retry::{to, Turn};
use super::step::IntentStep;
use crate::wallet::send::sent::Ledger;

pub const NOT_SENT: &str = "settle did not send the transaction; it may not have gone";

pub const UNFOLLOWED: &str = "settle answered, but no hash of what was signed is held to follow";

/// BROADCAST's answer for what was signed: its quote and its own hash,
/// the one recorded at Sign and followed, whatever hash settle answered.
pub fn after_broadcast(answer: Option<[u8; 32]>, signed: Option<(Quote, [u8; 32])>) -> IntentStep {
    match (answer, signed) {
        (Some(_), Some((_, hash))) => IntentStep::Sent(hash),
        (None, Some((quote, hash))) => IntentStep::Unsure { quote, hash },
        (Some(_), None) => IntentStep::Failed(UNFOLLOWED),
        (None, None) => IntentStep::Failed(NOT_SENT),
    }
}

/// GAS's pending nonce, read after an Unsure.
pub fn after_unsure(ledger: &mut Ledger, step: IntentStep, pending: u64) -> Turn {
    match step {
        IntentStep::Unsure { quote, hash } if pending > quote.nonce => to(IntentStep::Sent(hash)),
        IntentStep::Unsure { quote, hash } => {
            ledger.unused(&hash);
            to(IntentStep::Review { quote })
        }
        other => to(other),
    }
}

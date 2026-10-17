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
 * The publisher send a step follows (B8, the Lead's ruling on commit 5):
 * its fate read from the send ledger at each paint and tick, never kept
 * apart, under the hash of the bytes signed recorded at Sign, never
 * settle's answer (followed_say.rs). Replaced or dropped, it never lands;
 * dropped, only the guarded Sign offers it again, its nonce unused.
 */

use nonos_settle_proto::Quote;

use super::retry::{to, Turn};
use super::step::IntentStep;
use super::unsure::after_unsure;
use crate::wallet::send::sent::{Fate, Ledger};

#[path = "followed_say.rs"]
mod followed_say;
pub use followed_say::{echo_said, said};

/// The ledger's fate for the send `step` follows; None when it follows none.
pub fn fate_of(ledger: &Ledger, step: &IntentStep) -> Option<Fate> {
    let IntentStep::Sent(h) = step else { return None };
    ledger.get(h).map(|s| s.fate)
}

/// Whether the send followed may still take effect: not replaced, dropped, reverted.
pub fn may_land(ledger: &Ledger, step: &IntentStep) -> bool {
    use Fate::{Confirmed, Dropped, InBlock, Replaced};
    let off =
        |f| matches!(f, Replaced | Dropped | InBlock { ok: false, .. } | Confirmed { ok: false });
    matches!(step, IntentStep::Sent(_)) && !fate_of(ledger, step).is_some_and(off)
}

/// Whether the ledger judged the send followed dropped: its nonce is read.
pub fn dropped(ledger: &Ledger, step: &IntentStep) -> bool {
    fate_of(ledger, step) == Some(Fate::Dropped)
}

/// BROADCAST's answer when it names another hash than the one signed.
pub fn echo(answer: Option<[u8; 32]>, signed: Option<(Quote, [u8; 32])>) -> Option<[u8; 32]> {
    answer.filter(|a| signed.map(|(_, h)| h) != Some(*a))
}

/// GAS's pending nonce, read while a broadcast is unsure or once the send
/// followed is dropped: at most its review comes back, no press standing.
pub fn after_reread(
    ledger: &mut Ledger,
    step: IntentStep,
    signed: Option<(Quote, [u8; 32])>,
    pending: u64,
) -> Turn {
    match (step, signed) {
        (IntentStep::Sent(h), Some((quote, hash))) if h == hash && dropped(ledger, &step) => {
            after_unsure(ledger, IntentStep::Unsure { quote, hash }, pending)
        }
        (IntentStep::Unsure { .. }, _) => after_unsure(ledger, step, pending),
        _ => to(step),
    }
}

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

//! What the screens say once a broadcast is over.

use super::broadcast::Purpose;
use crate::wallet::send::outcome::Broadcast;
use crate::wallet::send::STAGE_SENT;
use crate::wallet::state::{State, VIEW_NOX, VIEW_SEND};

/// Move the screen the transaction was sent from on.
pub(super) fn show(state: &mut State, purpose: Purpose, result: Broadcast) {
    let went = result.follow().is_some();
    match purpose {
        Purpose::Pay => {
            /* Always the Sent screen: it says what came of this payment,
             * and nothing on it signs again. */
            state.send.stage = STAGE_SENT;
            state.send.draft = None;
            if went {
                state.send.amount = crate::wallet::num::Amount::new();
            }
            if state.view == VIEW_SEND {
                state.failure = refusal(result);
                state.scroll = 0;
            }
        }
        Purpose::Approve | Purpose::Stake | Purpose::Unstake => {
            /* The step moves on when the receipt says so (`send::follow`). */
            state.stake_draft = None;
            if state.view == VIEW_NOX {
                state.failure = if went { None } else { core::str::from_utf8(state.status).ok() };
            }
        }
    }
}

/// The reason the screen shows when the transaction did not go.
fn refusal(result: Broadcast) -> Option<&'static str> {
    match result {
        Broadcast::Sent(_) | Broadcast::Unknown(_) => None,
        Broadcast::Refused(why) | Broadcast::NotSent(why) => Some(why),
    }
}

/// The status line once the broadcast is over.
pub(super) fn said(purpose: Purpose, result: Broadcast) -> &'static [u8] {
    match (purpose, result) {
        (_, Broadcast::Refused(why)) | (_, Broadcast::NotSent(why)) => why.as_bytes(),
        (Purpose::Pay, Broadcast::Sent(_)) => b"payment sent",
        (Purpose::Pay, Broadcast::Unknown(_)) => b"payment may be sent, following its receipt",
        (Purpose::Approve, Broadcast::Sent(_)) => b"approval sent, waiting for its receipt",
        (Purpose::Approve, Broadcast::Unknown(_)) => b"approval may be sent, following its receipt",
        (Purpose::Stake, Broadcast::Sent(_)) => b"stake sent",
        (Purpose::Stake, Broadcast::Unknown(_)) => b"stake may be sent, following its receipt",
        (Purpose::Unstake, Broadcast::Sent(_)) => b"unstake sent",
        (Purpose::Unstake, Broadcast::Unknown(_)) => b"unstake may be sent, following its receipt",
    }
}

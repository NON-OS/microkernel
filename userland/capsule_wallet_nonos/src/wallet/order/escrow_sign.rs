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

//! A dispute's or a refund's signature (v1.4.51): SIGN_FROM_FRESH asked of
//! the Shield once, as a job; its raw transaction is broadcast next.

use alloc::vec::Vec;

use shield_wire::{field, unhex, OP_SIGN_FROM_FRESH};

use super::escrowing::Escrowing;
use super::send_step::SendStep;
use crate::wallet::state::State;

const NOT_SIGNED: &str = "The Shield did not sign the transaction from the refund address.";

fn escrowing(state: &mut State) -> Option<&mut Escrowing> {
    state.meter.purchase.as_deref_mut()?.escrow.as_mut()
}

/// Ask the Shield to sign, once. Whether the step moved.
pub fn sign(state: &mut State) -> bool {
    let ended = state.shield_ui.waiting.is_none();
    let Some(e) = escrowing(state) else { return false };
    let SendStep::Signing(ask) = &e.step else { return false };
    if e.signing {
        // The job ended without an answer applied: refused or lost.
        if ended {
            e.step = SendStep::Failed(NOT_SIGNED);
            return true;
        }
        return false;
    }
    let fields = ask.fields();
    e.signing = true;
    let words: Vec<&str> = fields.iter().map(|s| s.as_str()).collect();
    let started = crate::wallet::shield::job::start(state, OP_SIGN_FROM_FRESH, &words);
    if !started {
        escrowing(state).iter_mut().for_each(|e| e.step = SendStep::Failed(NOT_SIGNED));
    }
    true
}

/// SIGN_FROM_FRESH's answer: the raw transaction, to broadcast.
pub fn signed(state: &mut State, body: &str) {
    let raw = field(body, "raw").and_then(unhex);
    let Some(e) = escrowing(state) else { return };
    e.raw = raw.clone();
    e.step = raw.map_or(SendStep::Failed(NOT_SIGNED), SendStep::Broadcasting);
}

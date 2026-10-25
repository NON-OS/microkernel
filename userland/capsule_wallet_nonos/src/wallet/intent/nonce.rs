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
 * The nonce a publisher transaction is reviewed and signed at (B8, the
 * Lead's ruling on commit 4). Publisher and payment sign from one account
 * and share one nonce sequence, so both take the send ledger's one rule,
 * Ledger::next_nonce (send/sent_ledger.rs), as a payment's draft does
 * (send/draft_finish.rs): GAS's pending nonce, never under one past the
 * highest this account's sends may still use. GAS alone is not trusted.
 */

use nonos_settle_proto::{Gas, Quote};

use super::call::Publisher;
use super::ready::ready;
use super::review_bond::At;
use super::send::{PublisherSend, Signable};
use super::step::IntentStep;
use super::step_quote::after_balance;
use crate::wallet::send::sent::Ledger;

pub const NO_NONCE: &str = "this account has no nonce left to sign with; not signing";

/// `pending` is GAS's tx_nonce for the send's account.
pub fn nonce_for(ledger: &Ledger, send: &PublisherSend, pending: u64) -> Option<u64> {
    ledger.next_nonce(send.chain_id, &send.from, pending)
}

/// GAS's answer while the balance is read: reviewed at the shared nonce.
pub fn after_gas(ledger: &Ledger, send: &PublisherSend, step: IntentStep, g: &Gas) -> IntentStep {
    match after_balance(step, g.balance) {
        IntentStep::Review { quote } => match nonce_for(ledger, send, g.tx_nonce) {
            Some(nonce) => IntentStep::Review { quote: Quote { nonce, ..quote } },
            None => IntentStep::Failed(NO_NONCE),
        },
        other => other,
    }
}

/// At Sign, GAS read again: intent::ready at the shared nonce now, so a
/// review whose nonce another send took is refused (NONCE_MOVED).
pub fn sign_at(
    ledger: &Ledger,
    send: &PublisherSend,
    step: &IntentStep,
    chain_id: u64,
    pending: u64,
    at: At,
    held: impl Fn(&Publisher, &Quote) -> Option<&'static str>,
) -> Result<Signable, &'static str> {
    let nonce = nonce_for(ledger, send, pending).ok_or(NO_NONCE)?;
    ready(send, step, chain_id, nonce, at, held)
}

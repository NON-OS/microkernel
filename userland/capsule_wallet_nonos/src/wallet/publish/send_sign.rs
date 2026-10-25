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
 * A publisher transaction signed (B8), once Sign was pressed armed and
 * GAS read again: intent::sign_at's checks at the nonce both paths share,
 * then the main key's signature, as a payment's (send/confirm_sign.rs),
 * in the send ledger before it is broadcast (intent::after_sign). Its data
 * is the send's own calldata, built once; settle never supplies it (D5).
 */

use alloc::vec::Vec;

use nonos_nox_config::address;
use nonos_settle_proto::{Gas, Quote};

use crate::wallet::chain;
use crate::wallet::intent::{after_sign, quote_refusal, sign_at, IntentStep};
use crate::wallet::ipc::{sign_tx, TxRequest};
use crate::wallet::state::State;

const NO_NONCE: &str = "settle did not read the account's nonce, so nothing was signed";
const OTHER_ACCOUNT: &str = "this account is not the one the review was for; not signing";
const NOT_SIGNED: &str = "the keyring would not sign this transaction";
const NO_HASH: &str = "the signed transaction could not be hashed; not sending it";

/// GAS's answer after Sign: its nonce is the chain's pending one.
pub fn nonce_read(state: &mut State, gas: Option<Gas>) -> bool {
    let made = gas.ok_or(NO_NONCE).and_then(|g| signed(state, g.tx_nonce));
    let Some(s) = state.publisher.sending.as_mut() else { return true };
    let at = made.as_ref().map(|(_, q, h)| (*q, *h)).map_err(|w| *w);
    let turn = after_sign(&mut state.broadcast.sent, &s.send, at, nonos_libc::mk_uptime_ms());
    (s.raw, s.signed) = match made {
        Ok((raw, q, h)) => (Some(raw), Some((q, h))),
        Err(_) => (None, None),
    };
    super::send_retry::took(state, turn)
}

fn signed(state: &State, pending: u64) -> Result<(Vec<u8>, Quote, [u8; 32]), &'static str> {
    let s = state.publisher.sending.as_ref().ok_or(NOT_SIGNED)?;
    let (send, ledger) = (&s.send, &state.broadcast.sent);
    let at = |c| address(send.chain_id, c).ok();
    let held = |p: &_, q: &_| quote_refusal(p, send.chain_id, q);
    let t = sign_at(ledger, send, &s.step, chain::current().id, pending, &at, held)?;
    let IntentStep::Review { quote } = s.step else { return Err(NOT_SIGNED) };
    if send.from != state.acct.address {
        return Err(OTHER_ACCOUNT);
    }
    let (data, nonce, gas, max_priority, max_fee) =
        (&t.data[..], t.nonce, t.gas, t.max_priority, t.max_fee);
    let tx = TxRequest { to: t.to, value: t.value, data, nonce, gas, max_priority, max_fee };
    let v = &state.vault;
    let raw = sign_tx(v.keyring_port, v.owner_pid, v.wallet_id, send.chain_id, &tx);
    let raw = raw.map_err(|_| NOT_SIGNED)?;
    let mut hash = [0u8; 32];
    crate::wallet::tx_hash::tx_hash(&raw, &mut hash).then_some((raw, quote, hash)).ok_or(NO_HASH)
}

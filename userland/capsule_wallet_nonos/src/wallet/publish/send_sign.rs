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
 * the chain's pending nonce read: intent::ready's last checks, then the
 * main key's signature through the keyring, synchronously, as a payment's
 * (send/confirm_sign.rs). Its data is the send's own calldata, built once
 * when the send was made; settle never supplies it (D5).
 */

use alloc::vec::Vec;

use nonos_nox_config::address;
use nonos_settle_proto::Gas;

use crate::wallet::chain;
use crate::wallet::intent::{quote_refusal, ready, IntentStep, Signable};
use crate::wallet::ipc::{sign_tx, TxRequest};
use crate::wallet::state::State;

const NO_NONCE: &str = "settle did not read the account's nonce, so nothing was signed";
const OTHER_ACCOUNT: &str = "this account is not the one the review was for; not signing";
const NOT_SIGNED: &str = "the keyring would not sign this transaction";

/// GAS's answer after Sign: its nonce is the chain's pending one.
pub fn nonce_read(state: &mut State, gas: Option<Gas>) -> bool {
    let raw = gas.ok_or(NO_NONCE).and_then(|g| signed(state, g.tx_nonce));
    if let Some(s) = state.publisher.sending.as_mut() {
        (s.step, s.raw) = match raw {
            Ok(raw) => (IntentStep::Broadcasting, Some(raw)),
            Err(why) => (IntentStep::Failed(why), None),
        };
    }
    true
}

fn signed(state: &State, pending_nonce: u64) -> Result<Vec<u8>, &'static str> {
    let s = state.publisher.sending.as_ref().ok_or(NOT_SIGNED)?;
    let send = &s.send;
    let at = |c| address(send.chain_id, c).ok();
    let held = |p: &_, q: &_| quote_refusal(p, send.chain_id, q);
    let t: Signable = ready(send, &s.step, chain::current().id, pending_nonce, &at, held)?;
    if send.from != state.acct.address {
        return Err(OTHER_ACCOUNT);
    }
    let (data, nonce, gas, max_priority, max_fee) =
        (&t.data[..], t.nonce, t.gas, t.max_priority, t.max_fee);
    let tx = TxRequest { to: t.to, value: t.value, data, nonce, gas, max_priority, max_fee };
    let v = &state.vault;
    sign_tx(v.keyring_port, v.owner_pid, v.wallet_id, send.chain_id, &tx).map_err(|_| NOT_SIGNED)
}

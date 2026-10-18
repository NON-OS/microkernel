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
//! The payment typed on the form, checked before the network is asked.

use super::*;

/// Work out the payment typed on the form, or say in one sentence why it
/// cannot be made. Nothing here reaches the network.
pub fn plan(state: &State) -> Result<Plan, &'static str> {
    if state.vault.wallet_id == 0 || !state.acct.address_ready {
        return Err("Make or restore a wallet first.");
    }
    let recipient = crate::wallet::event::recipient(state)
        .ok_or("The address must be 0x and 40 hex digits.")?;
    if recipient == [0u8; 20] {
        return Err("That is the zero address. Nothing sent there can come back.");
    }
    /* A mixed-case address carries an EIP-55 checksum, and it must hold. */
    let typed = &state.send.to_hex;
    if crate::wallet::event::address_text::mixed_case(typed) {
        let k = crate::wallet::publish::Noted::default();
        let ok = crate::wallet::event::address_text::case_ok(&k, typed);
        if !k.answered() {
            return Err("The address's checksum could not be checked. Try again in a moment.");
        }
        if !ok {
            return Err("The capital letters in this address do not match its checksum, so a \
                 character is likely wrong. Check it against where it came from.");
        }
    }
    let c = chain::current();
    if recipient == c.nox || recipient == c.usdc {
        return Err("That is a token contract. Anything sent to it is lost: send to the \
             person's own address.");
    }
    if c.staking == Some(recipient) {
        return Err("That is the staking contract. Stake from Staking; a payment sent to it \
             is lost.");
    }
    let asset = state.send.token;
    let amount = state.send.amount.scaled(super::super::decimals(asset));
    if amount == 0 {
        return Err("Type an amount above zero.");
    }
    let have = held(state, asset).ok_or("The balance has not been read yet. Try again shortly.")?;
    if amount > have {
        return Err("That is more than this account holds.");
    }
    let (to, value, data) = match asset {
        ASSET_ETH => (recipient, amount, Vec::new()),
        ASSET_NOX => (c.nox, 0, transfer_data(&recipient, amount)),
        _ => (c.usdc, 0, transfer_data(&recipient, amount)),
    };
    Ok(Plan { purpose: Purpose::Pay, asset, recipient, amount, to, value, data, chain_id: c.id })
}

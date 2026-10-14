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

//! Moving the wallet between Ethereum mainnet and Sepolia.
//!
//! The address is the same on both, so nothing about the account changes.
//! Every figure read from the old network is dropped at once, so no balance,
//! nonce or fee from one chain is ever shown or signed against on the other,
//! and the link is checked again, since the new network is another host.

use super::State;

pub use super::forget_live::forget_live;

/* Said when a switch would leave something going out on the old network. */
pub const SWITCH_BUSY: &str = "A payment or a shield job is under way on this network. \
     Let it finish, then change the network.";

/* Ok(false) when the network picked is already this one. A broadcast or a
 * shield job under way holds the network: its answer belongs to the old
 * one, so the switch waits for it rather than leave it half followed. */
pub fn switch_network(state: &mut State, sepolia: bool) -> Result<bool, &'static str> {
    if crate::wallet::chain::is_sepolia() == sepolia {
        return Ok(false);
    }
    let sending = matches!(state.action, Some(crate::wallet::act::Action::Broadcast { .. }));
    if sending || state.shield_ui.waiting.is_some() {
        return Err(SWITCH_BUSY);
    }
    crate::wallet::chain::pick(sepolia);
    /* Kept, so the next boot opens on this network, on a boot that keeps anything. */
    if crate::wallet::vault::persistent() {
        let _ = crate::wallet::vault::remember_network(sepolia);
    }
    forget_live(state);
    /* The shield runs on one network: its store is closed on leaving it, and
     * opened at once on arriving where it runs, so its address is ready. */
    crate::wallet::shield::open::forget_network(state);
    if crate::wallet::chain::current().shield {
        crate::wallet::shield::open::ensure(state);
    } else {
        state.acct.receive_private = false;
    }
    Ok(true)
}

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
 * Where one publisher transaction is (its moves on a quote and a balance
 * are step_quote.rs), and Q8's two refusals: no start while a wallet send
 * is pending, and no signature once the chain's pending nonce is not the
 * one quoted, so a quote another send overtook is quoted again.
 */

use nonos_settle_proto::Quote;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IntentStep {
    Quoting,
    Funds(Quote),
    Review {
        quote: Quote,
    },
    Broadcasting,
    /// Settle did not confirm the broadcast: it may not have gone (retry.rs).
    Unsure {
        quote: Quote,
        hash: [u8; 32],
    },
    Sent([u8; 32]),
    Landed,
    Failed(&'static str),
}

pub const SEND_PENDING: &str = "a wallet send is still pending; wait for it to land first";
pub const NOT_OURS: &str = "that calldata is not one the wallet writes, not signing";
pub const NO_FUNDS: &str = "not enough ETH for this transaction's most network fee";
pub const NONCE_MOVED: &str = "the account's nonce moved since the quote; quote it again";
pub const NOT_REVIEWED: &str = "nothing reviewed to sign";

pub fn start(send_pending: bool) -> IntentStep {
    if send_pending {
        IntentStep::Failed(SEND_PENDING)
    } else {
        IntentStep::Quoting
    }
}

/// The quote to sign, read at Sign against the chain's pending nonce.
pub fn sign(step: &IntentStep, pending_nonce: u64) -> Result<Quote, &'static str> {
    match step {
        IntentStep::Review { quote } if quote.nonce == pending_nonce => Ok(*quote),
        IntentStep::Review { .. } => Err(NONCE_MOVED),
        _ => Err(NOT_REVIEWED),
    }
}

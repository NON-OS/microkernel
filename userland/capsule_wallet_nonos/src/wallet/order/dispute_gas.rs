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

//! A dispute's quote and its gas (v1.4.51): settle's INTENT held to its
//! bounds, then refundTo's ETH, from GAS, against the most it can cost.

use nonos_settle_proto::Quote;

use super::dispute::DisputeSend;
use super::dispute_quote::{fresh_ask, gas_size, quote_refusal};
use super::dispute_step::{DisputeStep, Disputed};

const TOO_COSTLY: &str = "The gas would cost more than 10 ETH at this fee; try again later.";

fn cost(q: &Quote) -> u128 {
    (q.gas_limit as u128).saturating_mul(q.max_fee)
}

impl Disputed<'_> {
    /// The quote held to its bounds, then refundTo's balance against its cost.
    pub fn after_quote(
        &self,
        send: DisputeSend,
        q: Quote,
        escrow: Option<[u8; 20]>,
        balance: u128,
    ) -> DisputeStep {
        if let Some(why) = quote_refusal(&q, escrow) {
            return DisputeStep::Failed(why);
        }
        let Some(size) = gas_size(&q) else { return DisputeStep::Failed(TOO_COSTLY) };
        match balance >= cost(&q) {
            true => DisputeStep::Signing(fresh_ask(&send, &q, self.chain_id)),
            false => DisputeStep::Gas { send, quote: q, size },
        }
    }

    /// GAS read while the withdraw lands: sign once refundTo holds the cost.
    pub fn after_balance(&self, step: DisputeStep, balance: u128) -> DisputeStep {
        match step {
            DisputeStep::Gas { send, quote, .. } if balance >= cost(&quote) => {
                DisputeStep::Signing(fresh_ask(&send, &quote, self.chain_id))
            }
            other => other,
        }
    }
}

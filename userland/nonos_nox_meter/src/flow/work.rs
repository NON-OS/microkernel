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

//! What a job is for: its chain, its first ask, the contracts it needs.

use nonos_nox_config::Contract;
use nonos_payment_proto::{Close, OpenStart, TopUp};

use super::ask::Ask;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Work {
    Open(OpenStart),
    TopUp(TopUp),
    Close(Close),
    /// The NOX the Shield pool credited `meter` (v1.4.17).
    Claim {
        chain_id: u64,
        meter: [u8; 20],
    },
}

impl Work {
    pub fn chain_id(&self) -> u64 {
        match self {
            Work::Open(s) => s.chain_id,
            Work::TopUp(t) => t.chain_id,
            Work::Close(c) => c.chain_id,
            Work::Claim { chain_id, .. } => *chain_id,
        }
    }

    /// The first ask: CLOSE reads GRACE, CLAIM what is claimable, the rest GAS.
    pub fn first(&self) -> Ask {
        match self {
            Work::Close(_) => Ask::Grace,
            Work::Claim { .. } => Ask::Claimable,
            _ => Ask::Gas,
        }
    }

    /// The config contracts the job needs deployed on its chain.
    pub fn needs(&self) -> &'static [Contract] {
        match self {
            Work::Open(_) | Work::TopUp(_) => &[Contract::NoxToken, Contract::Settlement],
            Work::Close(_) => &[Contract::Settlement],
            Work::Claim { .. } => &[Contract::ShieldPool],
        }
    }
}

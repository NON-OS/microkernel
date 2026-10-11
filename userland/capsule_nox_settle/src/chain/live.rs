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

//! Which config contracts settle will name as `to` or read: deployed, and
//! passed their checks. One that failed is not deployed (ENOENT) until the
//! next start; one whose check could not be asked is asked again on use.

use nonos_nox_config::{deployed, Contract, CONTRACTS, MAINNET, SEPOLIA};
use nonos_settle_proto::{EINVAL, ENOENT};

use super::check::check;
use crate::net::Rpc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Mark {
    Unknown,
    Good,
    Bad,
}

pub struct Live {
    pub(super) marks: [[Mark; CONTRACTS.len()]; 2],
}

pub(super) fn place(chain_id: u64, c: Contract) -> Result<(usize, usize), i32> {
    let chain = match chain_id {
        MAINNET => 0,
        SEPOLIA => 1,
        _ => return Err(EINVAL),
    };
    Ok((chain, CONTRACTS.iter().position(|x| *x == c).ok_or(ENOENT)?))
}

impl Live {
    pub fn new() -> Live {
        Live { marks: [[Mark::Unknown; CONTRACTS.len()]; 2] }
    }

    pub fn check_all(&mut self, rpc: &mut Rpc) {
        for chain in [MAINNET, SEPOLIA] {
            for c in CONTRACTS {
                let _ = self.usable(rpc, chain, c);
            }
        }
    }

    /// The address of `c` on `chain_id`, when it is deployed and passed.
    pub fn usable(&mut self, rpc: &mut Rpc, chain_id: u64, c: Contract) -> Result<[u8; 20], i32> {
        let (i, j) = place(chain_id, c)?;
        let at = deployed(chain_id, c).map_err(|_| ENOENT)?.address;
        if self.marks[i][j] == Mark::Unknown {
            let passed = check(rpc, chain_id, c, &at)?;
            self.marks[i][j] = if passed { Mark::Good } else { Mark::Bad };
        }
        (self.marks[i][j] == Mark::Good).then_some(at).ok_or(ENOENT)
    }
}

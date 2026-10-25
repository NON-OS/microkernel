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

//! LIST_TOKENS: what the meter pays in, read from nonos_nox_config. One NOX
//! row per chain the token is deployed on; a chain without it has no row.
//!
//!   count u32 | count x (symbol_len u8 | decimals u8 | settlement u16
//!   | flags u32 | chain_id u64 | contract[20] | symbol)

use alloc::vec::Vec;

use nonos_nox_config::{address, Contract, MAINNET, SEPOLIA};

use crate::server::Call;

/// Paid by cumulative voucher on a prefunded channel.
const SETTLEMENT_NOX_VOUCHER: u16 = 4;
const TOKEN_ENABLED: u32 = 1 << 0;
const TOKEN_ERC20: u32 = 1 << 2;

pub fn tokens(req: &Call<'_>) -> Vec<u8> {
    let rows: Vec<(u64, [u8; 20])> = [MAINNET, SEPOLIA]
        .into_iter()
        .filter_map(|chain| address(chain, Contract::NoxToken).ok().map(|a| (chain, a)))
        .collect();
    let mut out = Vec::with_capacity(4 + rows.len() * 39);
    out.extend_from_slice(&(rows.len() as u32).to_le_bytes());
    for (chain, contract) in rows {
        out.push(3);
        out.push(18);
        out.extend_from_slice(&SETTLEMENT_NOX_VOUCHER.to_le_bytes());
        out.extend_from_slice(&(TOKEN_ENABLED | TOKEN_ERC20).to_le_bytes());
        out.extend_from_slice(&chain.to_le_bytes());
        out.extend_from_slice(&contract);
        out.extend_from_slice(b"NOX");
    }
    req.answer(0, &out)
}

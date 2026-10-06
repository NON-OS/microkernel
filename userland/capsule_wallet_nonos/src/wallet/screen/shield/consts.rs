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
 * The Sepolia deployment the Shield screens name, written out in full so it
 * can be checked against the STARKs README's "Deployed" table rather than
 * trusted. The service reads the same pool; fees are never written here,
 * they are read from the pool's policy for each payment.
 */

pub const CHAIN_ID: &str = "11155111";
pub const POOL: &str = "0xaEe51E82965Ec1DeD870F3f4c248Ad4AdDc3e1cb";
pub const POOL_FROM_BLOCK: &str = "11,817,433";
pub const VERIFIER: &str = "0xDA9dD4A3e957AFD2179131273C93dabBA1186A44";
pub const REGISTRY: &str = "0xF6B5c3470eb7F1bdE3412E72Eff4235A4536a206";
pub const POLICY: &str = "0x660f66ab31Ca9919D9e1770FEDc88Ff2dd29CE59";
pub const NOX_TOKEN: &str = "0x3E5249A65CA513D5e11260222e0D26f46b465d36";
pub const FAUCET: &str = "0x871bc3AD5DA20c399d631817637cB5FB29eB04B4";

/* What the holder types to spend a note before it has matured. */
pub use crate::wallet::shield::actions::OVERRIDE;

pub const SEND_LEAD: &str = "The proof hides who paid whom and how much. \
     Deposits and withdrawals are public standard amounts.";

pub fn ticker(asset: u8) -> &'static str {
    crate::wallet::shield::actions::coin(asset)
}

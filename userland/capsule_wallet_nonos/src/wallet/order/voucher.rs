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
 * A voucher listing's channel (B8, v1.4.38): the market's terms. OPEN
 * 12/0's request is voucher_open.rs's.
 */

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Voucher {
    pub chain_id: u64,
    pub payee: [u8; 20],
    pub listing_id: [u8; 32],
    pub unit: [u8; 32],
    pub unit_price: u128,
    pub min_charge: u128,
}

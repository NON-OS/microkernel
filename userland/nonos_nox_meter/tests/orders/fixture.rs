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

//! An ORDER_NEW at NOW and what the plan is given.

use nonos_payment_proto::OrderNew;

pub const NOW: u64 = 1_800_000_000;
pub const NOX: [u8; 20] = [0x0a; 20];
pub const ESCROW: [u8; 20] = [0xe5; 20];
pub const FWD: [u8; 32] = [0xf0; 32];
/// The meter's order salt, and the wallet's refundSalt in the request.
pub const SALT: [u8; 32] = [0x51; 32];
pub const REFUND_SALT: [u8; 32] = [0x52; 32];

pub fn new(deadline: u64, amount: u128) -> OrderNew {
    OrderNew {
        chain_id: 11155111,
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [3; 32],
        payee: [4; 20],
        amount,
        chunk_root: [5; 32],
        content_len: 4096,
        cipher_root: [6; 32],
        key_commit: [7; 32],
        deadline,
        refund_to: [8; 20],
        refund_salt: REFUND_SALT,
    }
}

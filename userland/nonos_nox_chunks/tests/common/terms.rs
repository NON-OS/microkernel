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

//! One OrderTerms literal, for the calldata tests.

use nonos_nox_chunks::OrderTerms;

pub fn terms() -> OrderTerms {
    OrderTerms {
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [3; 32],
        payee: [4; 20],
        token: [0xA; 20],
        amount: 7,
        delivery_deadline: 100_000,
        dispute_window: 259_200,
        chunk_root: [5; 32],
        content_len: 9,
        cipher_root: [6; 32],
        key_commit: [7; 32],
        refund_commit: [8; 32],
        salt: [10; 32],
    }
}

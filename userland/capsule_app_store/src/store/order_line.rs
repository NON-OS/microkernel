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

//! What each of the escrow's states reads as in the store's status line.

/// The status line for an order in `state`; `plain`
/// once ORDER_DELIVERED has sealed its plaintext for the install.
pub fn order_line(state: u8, plain: bool) -> &'static [u8] {
    match (state, plain) {
        (0, _) => b"bought: fund the order in your wallet",
        (1, _) => b"bought: waiting for the seller to deliver",
        (2, false) => b"delivered: your wallet checks it",
        (2, true) => b"delivered and checked: ready to install",
        (3, _) => b"disputed: the seller's bond answers for it",
        (4, _) => b"paid to the seller",
        (5, _) => b"refunded to your wallet",
        _ => b"an order state this store does not know",
    }
}

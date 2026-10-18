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

/// A confirmed transfer of the token to the payee.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Payment {
    /// The paying account, from the transfer's first indexed argument.
    pub from: [u8; 20],
    /// What arrived, in the token's base units. NOX takes a fee on transfer,
    /// so this is the logged amount, never what the sender put in.
    pub amount: u128,
}

/// Why a receipt does not prove the payment.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ReceiptError {
    /// No `status` field, so the transaction's success is unknown.
    NoStatus,
    /// Transaction reverted (`status` `0x0`).
    Reverted,
    /// No transfer of the token to the payee for at least the price was found.
    NoMatchingTransfer,
}

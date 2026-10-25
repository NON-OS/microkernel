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

//! Where a purchase the store named stands, step by step.

use alloc::boxed::Box;

use nonos_payment_proto::{OpenConfirm, OpenStart, OrderMade, OrderNew};

pub enum Step {
    Consent,
    Offering(u32),
    Ordering(u32, Box<OrderNew>),
    /// The order is made: its deposit address waits for the NOX.
    Fund(Box<OrderMade>),
    /// A voucher channel: OPEN START's job, then CONFIRM until it is mined.
    Opening(u32, Box<OpenStart>),
    Confirming(Box<OpenConfirm>, u64),
    Opened([u8; 32]),
    /// The person chose the channel the listing already had.
    Kept([u8; 32]),
    Failed(&'static str),
}

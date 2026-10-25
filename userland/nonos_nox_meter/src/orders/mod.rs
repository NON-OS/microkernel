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

//! The meter's escrow orders (plan/21, plan/27, v1.4.20): each order's terms
//! and its committed refund, kept sealed under the bound root, never shown
//! but to the wallet's ORDER_REFUND.

mod delivered;
mod kept;
mod plan;
mod record;
mod table;
mod take;

pub use delivered::{deliver, Opened};
pub use kept::{KeptDelivered, MAX_KEPT};
pub use plan::order_plan;
pub use record::{Order, ORDER_LEN};
pub use table::{decode_orders, encode_orders, MAX_ORDERS, ORDERS_LEN};
pub use take::{offer_for, order_takes};

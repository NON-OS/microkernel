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

//! The order a purchase became (B9, v1.4.42): the names posted, then the
//! orderId READY hands back for exactly those names, followed by
//! order_watch.rs. A READY for other names is never this order.

use nonos_payment_proto::{Names, Ready};

#[derive(Default)]
pub struct Orders {
    /// The names this store posted at 19/0, for the order it follows.
    pub posted: Option<Names>,
    pub order: Option<Ready>,
    pub(super) job: Option<u32>,
    pub(super) at_ms: i64,
}

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

//! The order a purchase became (B9, v1.4.42): its names awaited back from
//! the wallet, then its orderId, followed by order_watch.rs.

use nonos_payment_proto::Ready;

#[derive(Default)]
pub struct Orders {
    /// Names went to the wallet and their orderId has not come back yet.
    pub awaiting: bool,
    pub order: Option<Ready>,
    pub(super) job: Option<u32>,
    pub(super) at_ms: i64,
}

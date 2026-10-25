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

use crate::server::context::Context;
use crate::server::reply::{sender, vouched};

/// Every other request names a slot in its first byte, one the sender must
/// have enabled (F4): a revival hands the same ids out again, and a stale
/// one reached another client's new device. A slot the multiplexer vouched
/// for (a shared device's claimant, a parked child's driver) is served too;
/// it is allocated, so a stale id still reaches nothing.
pub(super) fn sender_holds(ctx: &Context, body: &[u8]) -> bool {
    let (slots, max) = (&ctx.driver.slots, ctx.driver.layout.max_slots);
    body.first().is_some_and(|&slot| {
        slots.held_by(slot, max, sender()) || (vouched() && slots.is_allocated(slot, max))
    })
}

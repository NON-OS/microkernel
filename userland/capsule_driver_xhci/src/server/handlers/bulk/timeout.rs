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
//! A bulk transfer that timed out, its endpoint moved past its TRB.

use crate::error::{XhciError, XhciResult};
use crate::server::context::Context;
use crate::server::handlers::recover::recover_after;

/// A transfer that never completed leaves its TRB on the ring, and the next
/// one would queue behind it into the same page. The endpoint is stopped and
/// moved past it. A STALL is left to the class driver's OP_RESET_BULK.
pub(super) fn recover_timeout(
    ctx: &mut Context,
    at: (u8, bool),
    dir_in: bool,
    moved: XhciResult<u32>,
) {
    let (slot, second) = at;
    if moved != Err(XhciError::TransferCompletionTimeout) {
        return;
    }
    let max_slots = ctx.driver.layout.max_slots;
    let dci = match ctx.driver.slots.resources_mut(slot, max_slots).and_then(|r| r.pair_mut(second))
    {
        Some(p) if dir_in => p.dci_in,
        Some(p) => p.dci_out,
        None => return,
    };
    recover_after(ctx, slot, dci, XhciError::TransferCompletionTimeout, None);
}

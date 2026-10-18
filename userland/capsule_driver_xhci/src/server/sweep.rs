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

//! The slots of a class driver that exited or was restarted. Each slot is
//! its enabling client's (`SlotTable::set_owner`); once that pid is gone the
//! slot is disabled and what it held freed, so the ports it held come back
//! for the restarted driver, a new pid, to enumerate. Left alone they stayed
//! addressed or claimed until reboot, and Address Device answered E_BUSY.

use nonos_libc::mk_pid_alive;

use super::context::Context;
use super::handlers::disable_slot::release_slot;

/// Disable every slot of `ctx` whose owner has exited, but one `keep`
/// names: a shared device another live driver still claims an interface
/// of, or a child parked below a hub. One the controller does not disable
/// is tried again on the next sweep.
pub fn sweep_orphans(ctx: &mut Context, keep: impl Fn(u8) -> bool) {
    let max = ctx.driver.layout.max_slots;
    if ctx.driver.dead {
        return;
    }
    for slot in 1..=max {
        let Some(owner) = ctx.driver.slots.owner_of(slot, max) else { continue };
        if !mk_pid_alive(owner) && !keep(slot) {
            let _ = release_slot(ctx, slot);
        }
    }
}

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
use super::state::CommandRing;
use crate::constants::{COMMAND_RING_TRBS, TRB_BYTES};
use crate::trb::commands::noop_command;
use crate::trb::{read_volatile_at, write_volatile_at};
impl CommandRing {
    /// Turn the command at `issued` into a No Op, keeping its cycle bit, so
    /// a ring the controller starts later runs nothing it was meant to do
    /// (Linux `trb_to_noop`). For a command given up whose completion never
    /// came: the controller may not have taken it up, and the ring's
    /// dequeue may still name it. One it did take up is past the dequeue,
    /// and rewriting it changes nothing.
    pub fn cancel(&mut self, issued: u64) {
        let base = self.region.phys();
        let Some(offset) = issued.checked_sub(base) else { return };
        // The last TRB is the Link back to the start, never a command.
        let commands = ((COMMAND_RING_TRBS - 1) * TRB_BYTES) as u64;
        if offset % TRB_BYTES as u64 != 0 || offset >= commands {
            return;
        }
        let va = self.region.user_va() + offset;
        // SAFETY: `offset` is a whole TRB short of the Link, inside the
        // ring's own grant.
        let cycle = unsafe { read_volatile_at(va) }.get_cycle();
        // SAFETY: as above.
        unsafe { write_volatile_at(va, noop_command(cycle)) };
    }
}

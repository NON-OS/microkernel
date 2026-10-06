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

use super::run_command::run_command;
use crate::error::XhciResult;
use crate::rings::command::CommandRing;
use crate::rings::event::EventRing;
use crate::trb::commands::noop_command;
pub fn issue_noop_and_wait(
    op_doorbell_base: u64,
    intr_base: u64,
    cmd_ring: &mut CommandRing,
    evt_ring: &mut EventRing,
) -> XhciResult<()> {
    let trb = noop_command(cmd_ring.cycle() != 0);
    run_command(op_doorbell_base, intr_base, cmd_ring, evt_ring, trb).map(|_| ())
}

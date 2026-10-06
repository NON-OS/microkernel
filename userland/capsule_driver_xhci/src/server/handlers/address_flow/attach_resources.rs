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

use crate::controller::clear_dcbaa_slot;
use crate::server::context::Context;
use crate::slots::SlotResources;

/// Put the addressed slot's resources in the table. False, with the DCBAA
/// entry cleared, when the table refuses them.
pub(super) fn attach_resources(ctx: &mut Context, resources: SlotResources) -> bool {
    let slot = resources.slot_id;
    if !ctx.driver.slots.attach_addressed(resources, ctx.driver.layout.max_slots) {
        let _ = clear_dcbaa_slot(&ctx.driver.dcbaa, slot, ctx.driver.layout.max_slots);
        return false;
    }
    true
}

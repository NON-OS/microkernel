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

use super::claim::claim;
use super::irq::bind_raw as irq_bind_raw;
use crate::discover::find_ps2_aux;

/// The aux (mouse) record the driver claimed and the IRQ12 grant bound on
/// it, 0 when the line could not be bound.
#[derive(Clone, Copy)]
pub(super) struct Aux {
    pub device_id: u64,
    pub irq_grant_id: u64,
}

/// Claim the aux record and bind its line, or `None` when there is no aux
/// record or the claim fails. A line that will not bind leaves the mouse
/// polled, as the keyboard is without IRQ1: the serve loop drains the
/// controller on every pass either way. The caller releases `device_id` if
/// the keyboard then fails to come up.
pub(super) fn setup_aux() -> Option<Aux> {
    let aux = find_ps2_aux()?;
    let epoch = claim(aux.device_id).ok()?;
    let irq_grant_id = irq_bind_raw(aux, epoch).map_or(0, |out| out.grant_id);
    Some(Aux { device_id: aux.device_id, irq_grant_id })
}

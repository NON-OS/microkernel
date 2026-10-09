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

/// The aux (mouse) record the driver claimed and the IRQ12 grant bound on it.
#[derive(Clone, Copy)]
pub(super) struct Aux {
    pub device_id: u64,
    pub irq_grant_id: u64,
}

/// Claim the aux record and bind its line, or `None` when there is no aux
/// record or either step fails (a failed bind gives the claim back). The
/// caller releases `device_id` if the keyboard then fails to come up.
pub(super) fn setup_aux() -> Option<Aux> {
    let aux = find_ps2_aux()?;
    let epoch = claim(aux.device_id).ok()?;
    match irq_bind_raw(aux, epoch) {
        Ok(out) => Some(Aux { device_id: aux.device_id, irq_grant_id: out.grant_id }),
        Err(_) => {
            let _ = nonos_libc::mk_device_release(aux.device_id);
            None
        }
    }
}

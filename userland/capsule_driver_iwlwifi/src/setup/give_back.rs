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
//! A finished bring-up given back whole, as a failed one is, when the radio
//! finds a card this driver cannot run.

use super::grants::Grants;
use crate::driver::Driver;

/// Bus Master off, then every grant and the claim (`Grants::release`).
pub fn give_back(d: &Driver) {
    let (mmio, irq) = (d.mmio_grant, d.irq_grant);
    Grants { device_id: d.device_id, claim_epoch: d.claim_epoch, mmio, irq }.release();
}

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

//! Step 5, the firmware: PCRs 0, 1, 2 and 7 must be the values the
//! enrolment certificate fixed when the machine was enrolled. A firmware
//! update moves them, and means the machine is enrolled again.

use nsh_wire::Enrol;

use crate::consts::ENROLLED_PCRS;
use crate::{Doc, Refusal};

pub fn firmware(doc: &Doc<'_>, enrol: &Enrol<'_>) -> Result<(), Refusal> {
    for (i, want) in ENROLLED_PCRS.into_iter().zip(enrol.pcrs) {
        if doc.pcr(i) != Some(want) {
            return Err(Refusal::Firmware(i));
        }
    }
    Ok(())
}

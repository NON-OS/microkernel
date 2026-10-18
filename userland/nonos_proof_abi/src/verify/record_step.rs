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

//! Step 2: a final record, not saturated, sealed by a pinned vendor
//! supervisor with the caps the policy's mask calls for, naming an app.

use super::refusal::Refusal;
use crate::record::{Record, FLAG_FINAL};
use crate::trust::{supervisor_caps, Trust};

pub(super) fn record(rec: &Record, pins: &Trust, mask: u64) -> Result<(), Refusal> {
    let vendor = rec.authority == 0
        && pins.supervisors.contains(&rec.sup_meas)
        && rec.sup_caps == supervisor_caps(mask);
    if rec.flags != FLAG_FINAL || !vendor {
        return Err(Refusal::Supervisor);
    }
    if rec.app_id == [0; 32] {
        return Err(Refusal::AppId);
    }
    Ok(())
}

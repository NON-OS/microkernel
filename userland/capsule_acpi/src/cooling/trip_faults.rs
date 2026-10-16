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

//! A zone's trip points misordered by firmware (F-110b), said once per zone
//! and fault: which trip was dropped or kept, so the log shows why a zone
//! reached _CRT or never reaches _HOT.

use super::state::Zone;
use crate::sys::log::Line;
use nonos_aml::thermal::{Trips, CRT_UNDER_PSV, HOT_OVER_CRT, HOT_UNDER_PSV, HOT_UNDER_PSV_KEPT};

const SAID: [(u8, &[u8]); 4] = [
    (HOT_OVER_CRT, b"_HOT at or above _CRT: _HOT dropped"),
    (HOT_UNDER_PSV, b"_HOT at or below _PSV: dropped, _CRT guards"),
    (HOT_UNDER_PSV_KEPT, b"_HOT at or below _PSV, no _CRT above it: kept"),
    (CRT_UNDER_PSV, b"_CRT at or below _PSV: kept, a firmware fault"),
];

pub fn tell(z: &mut Zone, t: &Trips) {
    for (bit, what) in SAID {
        if t.faults & bit != 0 && z.told & bit == 0 {
            Line::new(b"thermal zone ").hex(u64::from(z.id)).str(b": ").str(what).end();
            z.told |= bit;
        }
    }
}

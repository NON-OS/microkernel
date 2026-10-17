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

//! A zone's trip points misordered by firmware (F-110b), or unread, and its
//! _TMP, an _ACx or _TZP unread (F-127, F-127b): said once per zone and fault, so the log shows why
//! a zone reached _CRT, never reaches _HOT, or runs its fans unasked.

use super::state::Zone;
use crate::sys::log::Line;
use nonos_aml::thermal::{CRT_UNDER_PSV, HOT_OVER_CRT, HOT_UNDER_PSV, HOT_UNDER_PSV_KEPT};
use nonos_aml::thermal::{UNREAD_AC, UNREAD_CRT, UNREAD_HOT, UNREAD_PSV, UNREAD_TMP, UNREAD_TZP};

// The order rule's bits are a byte; the unread ones above it.
const fn b(bit: u8) -> u16 {
    bit as u16
}

const SAID: [(u16, &[u8]); 10] = [
    (b(HOT_OVER_CRT), b"_HOT at or above _CRT: _HOT dropped"),
    (b(HOT_UNDER_PSV), b"_HOT at or below _PSV: dropped, _CRT guards"),
    (b(HOT_UNDER_PSV_KEPT), b"_HOT at or below _PSV, no _CRT above it: kept"),
    (b(CRT_UNDER_PSV), b"_CRT at or below _PSV: kept, a firmware fault"),
    (UNREAD_CRT, b"_CRT cut short or not believed: the last one kept"),
    (UNREAD_HOT, b"_HOT cut short or not believed: the last one kept"),
    (UNREAD_PSV, b"_PSV cut short or not believed: the last one kept"),
    (UNREAD_TMP, b"_TMP cut short or not believed: last reading kept, fans on"),
    (UNREAD_AC, b"an _ACx cut short or not believed: the last one kept"),
    (UNREAD_TZP, b"_TZP cut short or not believed: the last period kept"),
];

pub fn tell(z: &mut Zone, faults: u16) {
    for (bit, what) in SAID {
        if faults & bit != 0 && z.told & bit == 0 {
            Line::new(b"thermal zone ").hex(u64::from(z.id)).str(b": ").str(what).end();
            z.told |= bit;
        }
    }
}

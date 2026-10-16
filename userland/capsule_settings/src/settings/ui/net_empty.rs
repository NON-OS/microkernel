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

//! What an empty network list says: why the scan found nothing, from what
//! the driver counted while it listened.

use super::valbuf::ValBuf;
use crate::wifi::ScanStats;

/*
 * An empty list says where reception stopped, from the driver's scan counters:
 * frames the radio received on the channels it swept, and how many parsed as
 * beacons. No frames is a receiver that hears nothing; frames but no beacons is
 * a parse that fails. Before any sweep the plain line stands.
 */
pub fn empty_scan(b: &mut ValBuf, stats: ScanStats) {
    if stats.steps == 0 {
        b.push_str("No networks found");
        return;
    }
    b.push_str("No networks: ");
    b.push_dec(stats.raw);
    b.push_str(" frames, ");
    b.push_dec(stats.beacons);
    b.push_str(" beacons heard");
}

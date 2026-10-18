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

//! Whether this driver has its device.

use crate::disk::Disk;

pub enum Medium {
    /// Still looking: a block request is asked to come again.
    Scanning,
    Ready(Disk),
    /// The device failed a request. Before it serves another it is asked
    /// whether it is still on its port, ready, and of the size it was
    /// (scan/check.rs), so a stick pulled out or a card swapped in a reader
    /// is not addressed as the one that was bound.
    Checking(Disk),
    Absent,
    /// Nothing is served again this boot: the driver cannot tell what it
    /// would serve from the medium it served. The kernel chose its disk
    /// once, by what it read. A card swapped in a reader once the kernel
    /// held a sector, a size that is not the one served, a card taken out,
    /// a change said by the unit served while it was asked again, and a let
    /// go device with no serial number all end here (scan/check.rs,
    /// scan/ask.rs, scan/release.rs).
    Changed,
}

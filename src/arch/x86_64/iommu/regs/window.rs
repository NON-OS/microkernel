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

use super::cap::{fault_recording_count, fault_recording_offset};
use super::offsets::iotlb_offset;

/* Whether every register the kernel touches lies inside a mapped window of
`window` bytes. The IOTLB register and the fault-recording registers sit
where CAP and ECAP say, up to about 16 KiB into the unit, so a unit that
places them past the window must be refused rather than accessed. */
pub const fn registers_fit(cap: u64, ecap: u64, window: usize) -> bool {
    let faults_end = fault_recording_offset(cap) + fault_recording_count(cap) as usize * 16;
    iotlb_offset(ecap) + 8 <= window && faults_end <= window
}

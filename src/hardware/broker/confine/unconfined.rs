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

//! A claim no remapping unit can confine. Most devices then go ahead on
//! physical addresses, said and counted on the posture line. A device behind
//! an external-facing port (F-36's flag) does not: whoever reached the
//! connector chose it, and it is refused rather than handed all of memory.
//! This is the IOMMU half of F 26; a dock's NIC or disk works on a machine
//! whose remapping unit confines it, and only there.

use super::attach::Refused;
use super::table::say;
use crate::hardware::broker::table::flags_of;
use crate::hardware::broker::DEVICE_FLAG_EXTERNAL;
use crate::memory::iommu::DeviceAddress;

pub(super) fn unconfined(
    pid: u32,
    device_id: u64,
    address: DeviceAddress,
    why: &[u8],
) -> Result<(), Refused> {
    if flags_of(device_id).is_some_and(|f| f & DEVICE_FLAG_EXTERNAL != 0) {
        say(b"refused: behind an external-facing port, and nothing would confine it", pid, address);
        return Err(Refused);
    }
    say(why, pid, address);
    Ok(())
}

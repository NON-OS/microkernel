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

//! Whether the driver's stream runs, as its own answers say. A START the
//! driver refuses (driver.usb_audio0 before a headset is bound answers
//! E_NODEV) leaves it stopped, so the next step asks again; a WRITE_PCM
//! answered E_NODEV means the driver lost its device and dropped the
//! stream, which must be started again once it has one. Pure, so
//! audio_proto_proofs holds it.

use super::proto::E_NODEV;

/// Running after a START the driver answered `accepted`.
pub fn started(accepted: bool) -> bool {
    accepted
}

/// Still running after a WRITE_PCM the driver answered `status`.
pub fn still_running(status: i32) -> bool {
    status != E_NODEV
}

/// Whether the output is chosen again now: on the tidy tick, or at once when
/// the driver said it lost its device, so the fixed sink takes over without
/// waiting out the tick.
pub fn rechoose_now(tick: bool, lost: bool) -> bool {
    tick || lost
}

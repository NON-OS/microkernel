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

//! What port 1 shows once its device has answered the driver's last write.

use std::time::{Duration, Instant};

use nonos_devmodel::FakeBar;

use super::model::PORT1;
use crate::constants::{PORTSC_PED, PORTSC_PRC};

/// How long the model may take to answer the driver's acknowledgement.
const SETTLE: Duration = Duration::from_secs(5);

/*
 * Not asserted on the value the driver returned: it reads PORTSC back after
 * its own acknowledgement of PRC, and in memory that read is the write until
 * the model has answered it, which a model thread the host did not schedule
 * for the 50 ms of reset recovery has not; on silicon it is the register.
 * What the specification fixes is what the port shows once the device has
 * taken the acknowledgement: enabled, and the change bit down. A read that
 * lands between the driver's write and the model's answer sees the write
 * itself, so the wait is for the device-shown state, not for any value with
 * the change bit down. The model runs on its own thread, so the wait is
 * bounded by time, as the driver's own waits are: a count of reads ran out
 * in a few milliseconds when the host was busy and the model's thread had
 * not been scheduled yet.
 */
pub fn settles_enabled(bar: &FakeBar) -> bool {
    let deadline = Instant::now() + SETTLE;
    loop {
        if bar.wrote32(PORT1) & (PORTSC_PRC | PORTSC_PED) == PORTSC_PED {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        std::thread::yield_now();
    }
}

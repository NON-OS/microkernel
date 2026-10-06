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

use crate::constants::{cap_to, CSTS_CFS, CSTS_RDY};
use crate::error::NvmeError;

/// CAP.TO counts in units of 500 ms.
pub const CAP_TO_UNIT_MS: u64 = 500;

/// The least the driver waits, whatever CAP.TO says. A CAP.TO of zero would
/// leave no time at all, and a controller may report less than it takes;
/// five seconds is what every controller was given before CAP.TO was read.
pub const READY_FLOOR_MS: u64 = 5_000;

/// How long CSTS.RDY may take to follow CC.EN on a controller with this CAP:
/// CAP.TO, the spec's worst case for the controller, and never under the
/// floor. CAP.TO is eight bits, so no controller can ask for more than
/// 255 * 500 ms = 127.5 s; that bound is wall time, not a spin count.
pub const fn ready_timeout_ms(cap: u64) -> u64 {
    let spec = cap_to(cap) as u64 * CAP_TO_UNIT_MS;
    if spec < READY_FLOOR_MS {
        READY_FLOOR_MS
    } else {
        spec
    }
}

/// What one read of CSTS means while the driver waits for CSTS.RDY to follow
/// its write of CC.EN.
#[derive(Clone, Copy, Debug)]
pub enum ReadyStep {
    /// Not there yet and time remains: read CSTS again.
    Wait,
    /// CSTS.RDY now matches CC.EN.
    Reached,
    /// The wait is over and the controller did not get there.
    Failed(NvmeError),
}

/// Decide the wait from the controller's CAP, one CSTS read and the
/// milliseconds since the wait began. A controller gone from the bus reads
/// all ones and ends either wait at once, never passing for ready or for
/// disabled. CSTS.CFS (controller fatal status) ends the enable wait at once.
/// The disable wait is the reset that clears CFS (NVMe 2.0, 3.7.2), so there
/// it waits for RDY and CFS both to clear: a controller firmware or an
/// earlier boot left fatal is reset and served, not refused. A CFS still set
/// when that wait runs out is the controller staying fatal through the
/// reset, and is said so rather than called slow.
pub fn ready_step(cap: u64, csts: u32, want_ready: bool, elapsed_ms: u64) -> ReadyStep {
    let fatal = (csts & CSTS_CFS) != 0;
    if csts == u32::MAX || (want_ready && fatal) {
        return ReadyStep::Failed(NvmeError::UnsupportedController);
    }
    if !fatal && ((csts & CSTS_RDY) != 0) == want_ready {
        return ReadyStep::Reached;
    }
    if elapsed_ms >= ready_timeout_ms(cap) {
        let e = if fatal { NvmeError::ControllerFatal } else { NvmeError::ControllerTimeout };
        return ReadyStep::Failed(e);
    }
    ReadyStep::Wait
}

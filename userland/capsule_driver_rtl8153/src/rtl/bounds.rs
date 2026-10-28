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

//! Every wait, in one place. Linux v6.12 looks longer at most of them
//! (autoload and the PHY's state 500 times 20 ms); here the waits a bind
//! makes total under 9.5 s, inside the 15 s bind budget (nonos_usbnet
//! xhci/budget.rs), and the ones made on a link change under 0.2 s, since
//! they run on the thread that answers net.core.

use super::op::Wait;

const fn wait(tries: u16, every_ms: u16, fail: Option<&'static str>) -> Wait {
    Wait { tries, every_ms, fail }
}

/// Reads of the version register before the chip is given up as silent.
pub const VERSION_TRIES: usize = 3;

/// The chip's firmware loading its settings after power-on: 4 s.
pub const AUTOLOAD: Wait = wait(200, 20, Some("chip never finished loading its settings"));
/// The PHY leaving its first state: 2 s, gone on from if it does not.
pub const PHY_SETTLED: Wait = wait(100, 20, None);
/// The PHY reaching LAN-on after power-up: 2 s.
pub const PHY_ON: Wait = wait(100, 20, Some("PHY never came on"));
/// ALDPS taking effect off: 20 ms.
pub const ALDPS_OFF: Wait = wait(20, 1, None);
/// The MAC's self-clearing reset: 100 ms.
pub const MAC_RESET: Wait = wait(100, 1, Some("MAC reset never finished"));
/// The buffer manager's link list, ready after a reset: 100 ms.
pub const LINK_LIST: Wait = wait(100, 1, Some("buffer link list never ready"));
/// The PHY's own reset, then autonegotiation starts: 1 s.
pub const PHY_RESET: Wait = wait(50, 20, None);
/// On link down, each FIFO draining and the reset after: 20 ms each.
pub const DRAIN: Wait = wait(20, 1, None);
/// The 8153B's flow-control task held off between two writes.
pub const FW_TASK_MS: u16 = 2;

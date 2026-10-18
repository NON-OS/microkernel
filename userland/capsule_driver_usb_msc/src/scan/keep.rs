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

//! A device the check (`check`) keeps after a request it failed: served on
//! as it was, or its medium served afresh, which raises the generation the
//! kernel reads (`asked`). A stick that timed out every READ yet passed
//! every check was kept for the boot: each request cost the kernel its
//! whole wait and a refusal, and no disk was ever chosen. Now one kept
//! after `FAILED_IN_A_ROW` failed requests is let go instead (`release`).
//! One with a serial number is searched for again. Let go time after time,
//! its port waits (ports.rs: 5 s, doubling to 80 s), and the kernel is told
//! there is no device until the wait ends; then it is bound again, so a
//! stick that still times out costs one request's 26 s in about every 107 s,
//! and the kernel's disk selection gets past it in between. One without a
//! serial number ends serving.

use super::identity::Identity;
use super::medium::Medium;
use super::scanner::Scanner;
use crate::disk::Disk;
use crate::protocol::E_IO;
use crate::state::State;

/// Failed requests in a row the check keeps a device after, none moving
/// the kernel's data between them (state/held.rs): as many transport
/// failures as an asking takes (pace.rs), so one stray timeout or medium
/// error does not cost the disk, and three of a request's 26 s at most.
const FAILED_IN_A_ROW: u8 = 3;

impl Scanner {
    /// Serve `disk` on, or afresh as `afresh` when that is given.
    pub(super) fn keep(
        &mut self,
        xhci: u32,
        disk: Disk,
        afresh: Option<Identity>,
        state: &mut State,
    ) {
        if state.kept_after_failure() >= FAILED_IN_A_ROW {
            return self.let_go(xhci, disk, E_IO);
        }
        match afresh {
            Some(id) => self.serve(disk, id),
            None => self.medium = Medium::Ready(disk),
        }
    }
}

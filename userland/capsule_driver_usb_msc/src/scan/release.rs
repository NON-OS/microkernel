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

//! What becomes of a bound device the check (`check`) does not keep: let go
//! for the search to find again, or the end of serving for this boot when
//! nothing would tell what is found after from what was served.

use super::medium::Medium;
use super::report::{Report, Stage};
use super::scanner::Scanner;
use crate::disk::Disk;
use crate::protocol::{E_NODEV, E_NOMEDIUM};
use crate::xhci::disable_slot;

impl Scanner {
    /// Give the slot back and start the search over. The port waits as one
    /// whose device failed, so a stick that fails every request backs off.
    /// One let go without leaving its port is on it still, so that port
    /// alone is asked (`search`): a stick of its make plugged in beside it,
    /// even one with its serial number, is not taken for it. A card taken
    /// out, or a device with no serial number, ends serving instead: the
    /// card put back, or the device that comes back, may be another of the
    /// same size, and was served as the kernel's disk.
    pub(super) fn let_go(&mut self, xhci: u32, disk: Disk, e: i32) {
        if e == E_NOMEDIUM || !self.served.is_some_and(|s| s.has_serial()) {
            return self.end(xhci, disk, e);
        }
        disable_slot(xhci, disk.slot);
        self.ports.failed(disk.port, false);
        let gone = e == E_NODEV;
        self.report = Report::at(if gone { Stage::Gone } else { Stage::NotReady }, disk.port, e);
        self.left_on = (!gone).then_some(disk.port);
        self.restart();
    }

    /// Give the slot back and serve nothing more this boot: block ops are
    /// refused. `e` says why: `E_NOMEDIUM` for a card taken out.
    pub(super) fn end(&mut self, xhci: u32, disk: Disk, e: i32) {
        disable_slot(xhci, disk.slot);
        self.report = Report::at(Stage::Changed, disk.port, e);
        (self.medium, self.searching) = (Medium::Changed, false);
    }
}

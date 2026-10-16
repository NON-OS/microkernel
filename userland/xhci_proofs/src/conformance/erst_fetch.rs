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

//! The event ring's start (xHCI 1.2 section 5.5.2.3.2): writing ERSTBA has
//! the controller fetch ERST[0], and qemu-xhci fetches it at the write. With
//! Bus Master off the fetch master-aborts, and the controller shows Host
//! Controller Error until it is reset: QEMU sets HCE and answers no command.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use nonos_devmodel::FakeBar;
use nonos_libc::{device_read64, pci_command_reg, DmaFault, PciCommandReg};

use super::bringup_model::{ERSTBA, STS};
use super::catch_up::CatchUp;
use super::model::wrote64;
use crate::constants::{USBCMD_HCRST, USBSTS_HCE};

pub struct ErstFetch {
    command: Arc<PciCommandReg>,
    catch_up: CatchUp,
    erstba: AtomicU64,
    failed: AtomicBool,
    /// Each look's time from its ERSTBA read to its fetch (`slowed`).
    slow: Duration,
}

impl ErstFetch {
    /// For the controller the calling thread's driver brings up: built on
    /// that thread, it sees the Bus Master bit that driver writes.
    pub fn on_this_thread() -> Self {
        let (command, catch_up) = (pci_command_reg(), CatchUp::on_this_thread());
        let (erstba, failed) = (AtomicU64::new(0), AtomicBool::new(false));
        Self { command, catch_up, erstba, failed, slow: Duration::ZERO }
    }

    /// Each look `slow` from its ERSTBA read to its fetch (`erst_fetch_tests`).
    pub fn slowed(self, slow: Duration) -> Self {
        Self { slow, ..self }
    }

    /// One look after the driver wrote `cmd`: HCRST clears the error, a new
    /// ERSTBA is fetched, and HCE is in USBSTS before `CatchUp` counts the look.
    pub fn turn(&self, bar: &FakeBar, cmd: u32) -> u32 {
        if cmd & USBCMD_HCRST != 0 {
            self.failed.store(false, Ordering::SeqCst);
        }
        let erstba = wrote64(bar, ERSTBA);
        std::thread::sleep(self.slow);
        if self.erstba.swap(erstba, Ordering::SeqCst) != erstba
            && device_read64(&self.command, erstba) == Err(DmaFault::MasterAbort)
        {
            self.failed.store(true, Ordering::SeqCst);
        }
        let hce = if self.failed.load(Ordering::SeqCst) { USBSTS_HCE } else { 0 };
        bar.present32(STS, (bar.wrote32(STS) & !USBSTS_HCE) | hce);
        self.catch_up.looked();
        hce
    }
}

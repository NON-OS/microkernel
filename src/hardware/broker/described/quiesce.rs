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

use super::interface::Interface;
use super::state::DESCRIBED;
use super::stop_regs::{halt_xhci, reset_ahci, reset_sdhci};

/// Stop a described device's DMA, as clearing Bus Master Enable does for a
/// PCI function, before its frames are freed. False for a device this
/// module did not publish, which is the caller's to stop.
pub(in crate::hardware::broker) fn stop(device_id: u64) -> bool {
    let all = DESCRIBED.read();
    let Some(entry) = all.iter().find(|e| e.device_id == device_id) else {
        return false;
    };
    let Some(regs) = entry.regs else {
        return true;
    };
    let stopped = match entry.interface {
        Interface::Sdhci => reset_sdhci(regs),
        Interface::Xhci => halt_xhci(regs),
        Interface::Ahci => reset_ahci(regs),
    };
    if !stopped {
        crate::sys::serial::print(b"[BROKER] described device ");
        crate::sys::serial::print_hex(device_id);
        crate::sys::serial::println(b" did not stop before its frames are freed");
    }
    true
}

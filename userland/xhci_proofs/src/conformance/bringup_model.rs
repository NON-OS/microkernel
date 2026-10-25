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

//! A controller window `setup::run` can bring up; how it behaves once
//! running is `bringup_device`.

use std::sync::Arc;

use nonos_devmodel::FakeBar;

use super::model::CAP_LEN;
use crate::constants::{
    CAPLENGTH, DBOFF, ERSTBA_LO, HCCPARAMS1, HCSPARAMS1, HCSPARAMS2, RTSOFF, USBCMD, USBSTS,
    USBSTS_HCH,
};
use crate::discover::Found;

/// The window: runtime registers at 0x1000 and doorbells at 0x2000, each
/// with the page the bring-up asks to be mapped behind it.
pub const WINDOW: usize = 0x3000;
/// PAGESIZE, the operational register naming the page sizes supported.
pub const PAGESIZE: usize = 0x08;
/// The scratchpad buffers the controller asks for.
const SCRATCHPADS: u32 = 4;
pub const CMD: usize = CAP_LEN + USBCMD as usize;
pub const STS: usize = CAP_LEN + USBSTS as usize;
/// Interrupter 0's ERSTBA, its register set 0x20 into the runtime page.
pub const ERSTBA: usize = 0x1000 + 0x20 + ERSTBA_LO as usize;

/// 8 slots, 2 ports, 4 scratchpads, 64-bit addressing, 4 KiB pages.
pub fn bringup_controller() -> Arc<FakeBar> {
    let bar = Arc::new(FakeBar::new(WINDOW));
    bar.present32(CAPLENGTH as usize, CAP_LEN as u32);
    bar.present32(HCSPARAMS1 as usize, (2 << 24) | 8);
    bar.present32(HCSPARAMS2 as usize, SCRATCHPADS << 27);
    bar.present32(HCCPARAMS1 as usize, 0x1);
    bar.present32(RTSOFF as usize, 0x1000);
    bar.present32(DBOFF as usize, 0x2000);
    bar.present32(CAP_LEN + PAGESIZE, 1);
    bar.present32(STS, USBSTS_HCH);
    bar
}

pub fn found() -> Found {
    Found {
        device_id: 7,
        bar0_size: WINDOW as u64,
        vendor: 0x1b36,
        device: 0x000d,
        on_pci: true,
        spi: 0,
    }
}

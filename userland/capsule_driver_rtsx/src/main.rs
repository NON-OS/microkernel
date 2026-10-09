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

//! driver.rtsx0: the Realtek PCIe SD card reader (RTS5227 and RTS522A, the
//! readers in many HP and Lenovo laptops), compared with Linux's
//! drivers/misc/cardreader/rtsx_pcr.c, rts5227.c and
//! drivers/mmc/host/rtsx_pci_sdmmc.c. It takes the reader from the broker,
//! brings the chip up, and watches the slot: a card that arrives is
//! identified and its first block read, each step said under "rtsx:".

#![no_std]
#![no_main]

extern crate alloc;

mod card;
mod chip;
mod clock;
mod engine;
mod error;
mod hw;
mod init;
mod log;
mod regs;
mod report;
mod sd;
mod setup;
mod watch;
mod wire;

use nonos_libc::{bring_up, heap_init, mk_exit, EXIT_ABSENT, EXIT_GAVE_UP};

use crate::error::RtsxError;
use crate::setup::{Driver, Found};

/// # Safety
///
/// The process entry point, entered once by the kernel's capsule loader.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    let Some(found) = setup::find() else {
        mk_exit(EXIT_ABSENT);
    };
    let Ok(mut drv) = bring_up(b"driver.rtsx0", || start(&found)) else {
        mk_exit(EXIT_GAVE_UP);
    };
    report::up(&drv);
    watch::run(&mut drv)
}

fn start(found: &Found) -> Result<Driver, &'static str> {
    let text = |e: RtsxError| core::str::from_utf8(e.name()).unwrap_or("rtsx bring-up failed");
    let mut drv = setup::run(found).map_err(|e| {
        log::failed(b"taking the reader", e);
        text(e)
    })?;
    init::init_hw(&mut drv).map_err(text)?;
    Ok(drv)
}

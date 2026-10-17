// NONOS Operating System (AGPL-3.0-or-later)

//! The register work that brackets the firmware download: halt the on-chip 8051,
//! point the transfer queues at the download path, reset the WL platform, then
//! after the sections land restore the queues, close out the download and
//! release the 8051, and confirm it reports ready. This is rtw88
//! `download_firmware_reg_backup`, `download_firmware_reset_platform`,
//! `wlan_cpu_enable`, `download_firmware_end_flow` and
//! `download_firmware_validate` in `mac.c`, reimplemented over the `Mmio` trait
//! so the exact register program is checked against a modeled device.

mod backup;
mod cpu;
mod kinds;
mod run;

pub use kinds::*;
pub use run::{abort, begin, finish};

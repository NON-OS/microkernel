// NONOS Operating System (AGPL-3.0-or-later)

//! The firmware download engine: section split, reserved-page staging,
//! DDMA, the 8051's halt and start, as rtw88 runs them for every 11ac chip
//! (mac.c `__rtw_download_firmware`), and the RTL8821CE's. The header parse
//! is this chip's (`header`): the same layout, the 8822C's signature.

#[path = "../../../capsule_driver_rtl8821ce/src/fw/ddma/mod.rs"]
pub mod ddma;
#[path = "../../../capsule_driver_rtl8821ce/src/fw/dma/mod.rs"]
pub mod dma;
#[path = "../../../capsule_driver_rtl8821ce/src/fw/download.rs"]
mod download;
pub mod header;
#[path = "../../../capsule_driver_rtl8821ce/src/fw/prep/mod.rs"]
pub mod prep;
#[path = "../../../capsule_driver_rtl8821ce/src/fw/regs.rs"]
pub mod regs;
#[path = "../../../capsule_driver_rtl8821ce/src/fw/rsvd/mod.rs"]
pub mod rsvd;
#[path = "../../../capsule_driver_rtl8821ce/src/fw/sections.rs"]
pub mod sections;
#[path = "../../../capsule_driver_rtl8821ce/src/fw/txdesc.rs"]
pub mod txdesc;

pub use download::{download, DownloadError};
pub use sections::MAX_CHUNK;

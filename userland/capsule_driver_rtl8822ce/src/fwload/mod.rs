// NONOS Operating System (AGPL-3.0-or-later)

//! The firmware download on the card. The download itself (DMA regions,
//! reserved-page staging, DDMA, the 8051's start) is the same on every rtw88
//! 11ac PCIe chip and is the RTL8821CE's; the image is the 8822C's (`image`).

pub mod image;
#[path = "../../../capsule_driver_rtl8821ce/src/fwload/load.rs"]
mod load;
#[path = "../../../capsule_driver_rtl8821ce/src/fwload/map.rs"]
mod map;
#[path = "../../../capsule_driver_rtl8821ce/src/fwload/say.rs"]
mod say;

pub use load::{load, FwLoadError};
pub(crate) use map::map_dma32;

// NONOS Operating System (AGPL-3.0-or-later)

//! The MAC: its init program (`tables`, the 8822C's `rtw8822c_mac_init`) and
//! its transmit/receive engines (`trx`, with the 8822C's FIFO and page counts
//! in `trx::pages`). The operation table, its executor, the station address
//! and the DMA burst are the RTL8821CE's, the same on every rtw88 11ac chip.

#[path = "../../../capsule_driver_rtl8821ce/src/mac/address.rs"]
mod address;
#[path = "../../../capsule_driver_rtl8821ce/src/mac/burst.rs"]
mod burst;
#[path = "../../../capsule_driver_rtl8821ce/src/mac/op.rs"]
pub mod op;
#[path = "../../../capsule_driver_rtl8821ce/src/mac/run.rs"]
mod run;
mod tables;
mod trx;

pub use address::set_address;
pub use burst::set_dma_burst;
pub use run::run_mac_table;
pub use tables::MAC_INIT;
pub use trx::{init_trx_cfg, quiesce, reset_trx_dma, RSVD_BOUNDARY};

// NONOS Operating System (AGPL-3.0-or-later)

//! The transmit and receive engines switched on: rtw88 `rtw_init_trx_cfg`
//! (mac.c), which is chip-independent once the FIFO sizes and page counts are
//! given. The program is the RTL8821CE's; `pages` holds the 8822C's numbers,
//! from which `bounds` derives the reserved-page boundary and the H2C queue.

#[path = "../../../../capsule_driver_rtl8821ce/src/mac/trx/bounds.rs"]
mod bounds;
#[path = "../../../../capsule_driver_rtl8821ce/src/mac/trx/engines.rs"]
mod engines;
pub mod pages;
#[path = "../../../../capsule_driver_rtl8821ce/src/mac/trx/queues.rs"]
mod queues;
#[path = "../../../../capsule_driver_rtl8821ce/src/mac/trx/regs.rs"]
mod regs;
#[path = "../../../../capsule_driver_rtl8821ce/src/mac/trx/reset.rs"]
mod reset;
#[path = "../../../../capsule_driver_rtl8821ce/src/mac/trx/stop.rs"]
mod stop;

pub use bounds::RSVD_BOUNDARY;
pub use engines::init_trx_cfg;
pub use reset::reset_trx_dma;
pub use stop::quiesce;

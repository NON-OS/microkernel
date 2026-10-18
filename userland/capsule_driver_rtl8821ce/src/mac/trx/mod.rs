// NONOS Operating System (AGPL-3.0-or-later)

//! Transmit/receive configuration: the step that actually switches the MAC's
//! DMA and receive engines on. This is rtw88 `rtw_init_trx_cfg` for the 8821c on
//! PCIe (`mac.c` `txdma_queue_mapping`, `priority_queue_cfg` and `init_h2c`),
//! reimplemented register-for-register. Without it the register table configures
//! the receiver but never enables it, so the radio is fully brought up yet hears
//! nothing. The page counts and reserved-page boundary are derived from the
//! chip's FIFO sizes exactly as rtw88 derives them, so they read as arithmetic
//! rather than magic. The whole program is checked against a modeled device in
//! `rtl8821ce_proofs`.

mod bounds;
mod engines;
mod pages;
mod queues;
mod regs;
mod reset;
mod stop;

pub use engines::init_trx_cfg;
pub use reset::reset_trx_dma;
pub use stop::quiesce;

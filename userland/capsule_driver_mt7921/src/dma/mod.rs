/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The host side of the WFDMA: grants, rings, and the engine's switches.

mod access;
pub mod desc;
mod grant;
pub mod layout;
mod prefetch;
pub mod ring;
pub mod rx;
mod tx;
pub mod wfdma;

pub use grant::Grant;
pub use ring::Ring;
pub use rx::Rx;

use crate::hw::Regs;
use layout::{Shape, ALL, DESC_PAGE, TOKENS, TOKEN_BUF};

/// Every ring the driver runs, and the frame buffers the data ring's TXPs
/// point at.
pub struct Dma {
    pub tx_data: Ring,
    pub tx_fwdl: Ring,
    pub tx_wm: Ring,
    pub rx_mcu: Ring,
    pub rx_data: Ring,
    pub rx_wa: Ring,
    pub tokens: Grant,
}

impl Dma {
    /// Map every ring for the claim; `None` if the broker refused any part.
    pub fn map(device_id: u64, epoch: u64) -> Option<Dma> {
        let page = Grant::map(device_id, epoch, DESC_PAGE)?;
        let ring = |i: usize| {
            let s: Shape = ALL[i];
            let desc = page.sub(layout::desc_offset(i), s.n as usize * desc::DESC_SIZE)?;
            let bufs = Grant::map(device_id, epoch, s.n as usize * s.stride)?;
            Ring::new(desc, bufs, s.stride, s.n, s.reg)
        };
        Some(Dma {
            tx_data: ring(0)?,
            tx_fwdl: ring(1)?,
            tx_wm: ring(2)?,
            rx_mcu: ring(3)?,
            rx_data: ring(4)?,
            rx_wa: ring(5)?,
            tokens: Grant::map(device_id, epoch, TOKENS * TOKEN_BUF)?,
        })
    }

    /// Every ring reset and every receive ring armed, as `mt7921_dma_init`
    /// leaves them before `mt792x_dma_enable`.
    pub fn reset(&mut self, regs: &Regs) {
        for r in [&mut self.tx_data, &mut self.tx_fwdl, &mut self.tx_wm] {
            r.reset(regs);
        }
        for r in [&mut self.rx_mcu, &mut self.rx_data, &mut self.rx_wa] {
            r.reset(regs);
            r.fill(regs);
        }
    }
}

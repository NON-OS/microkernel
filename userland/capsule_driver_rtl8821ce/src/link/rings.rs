// NONOS Operating System (AGPL-3.0-or-later)

//! Both rings pointed at, and the engines switched on only after that.

use crate::fw::dma::DmaMem;
use crate::regs::Mmio;
use crate::rx::program as rx_program;
use crate::tx::program as tx_program;

/*
 * Point the card at both rings and switch its transmit and receive engines on.
 * rtw88 sets the ring addresses in `rtw_hci_setup`, before `rtw_mac_init`
 * writes REG_CR: switched on first, the receive DMA would write frames to
 * whatever ring address the register held, zero after power-on or the last
 * boot's ring after a warm reboot. Returns `init_trx_cfg`'s answer.
 */
pub fn engines_on<M: Mmio, D: DmaMem>(mmio: &M, tx_ring: &D, rx_ring: &D, rx_dev: u64) -> bool {
    // rtw88 sets the DMA burst sizes before the ring addresses (pci.c:403).
    crate::mac::set_dma_burst(mmio);
    program_rings(mmio, tx_ring, rx_ring, rx_dev);
    crate::mac::init_trx_cfg(mmio)
}

/// Both rings programmed, the receive slots armed against the buffers at
/// `rx_dev`, and the DMA interface reset so the card starts from slot zero.
pub(super) fn program_rings<M: Mmio, D: DmaMem>(mmio: &M, tx_ring: &D, rx_ring: &D, rx_dev: u64) {
    tx_program(mmio, tx_ring.device_addr());
    rx_program(mmio, rx_ring, rx_dev);
    crate::mac::reset_trx_dma(mmio);
}

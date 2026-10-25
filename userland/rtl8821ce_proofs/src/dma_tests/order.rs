// NONOS Operating System (AGPL-3.0-or-later)
//! Bring-up points the card at both rings before REG_CR switches the receive
//! DMA on, as rtw88 does (`rtw_hci_setup` before `rtw_mac_init`).

use std::cell::RefCell;

use crate::fw::dma::DmaMem;
use crate::link::engines_on;
use crate::regs::Mmio;
use crate::rx::regs::REG_RXBD_DESA_MPDUQ;
use crate::tx::regs::REG_TXBD_DESA_BEQ;

const REG_CR: usize = 0x0100;
const HCI_RXDMA_EN: u32 = 1 << 1;

// Reads zero (the link-list table done, the H2C ring free); every write kept.
#[derive(Default)]
struct Card(RefCell<Vec<(usize, u32)>>);

impl Card {
    fn first(&self, f: impl Fn(usize, u32) -> bool) -> Option<usize> {
        self.0.borrow().iter().position(|&(o, v)| f(o, v))
    }
}

impl Mmio for Card {
    fn read8(&self, _: usize) -> u8 {
        0
    }
    fn write8(&self, off: usize, val: u8) {
        self.0.borrow_mut().push((off, val as u32));
    }
    fn read16(&self, _: usize) -> u16 {
        0
    }
    fn write16(&self, off: usize, val: u16) {
        self.0.borrow_mut().push((off, val as u32));
    }
    fn read32(&self, _: usize) -> u32 {
        0
    }
    fn write32(&self, off: usize, val: u32) {
        self.0.borrow_mut().push((off, val));
    }
}

struct Ring(u64);

impl DmaMem for Ring {
    fn capacity(&self) -> usize {
        1 << 16
    }
    fn device_addr(&self) -> u64 {
        self.0
    }
    fn write_bytes(&self, _: usize, _: &[u8]) {}
}

#[test]
fn the_receive_dma_starts_only_after_both_rings_are_set() {
    let card = Card::default();
    assert!(engines_on(&card, &Ring(0x1000_0000), &Ring(0x3000_0000), 0x4000_0000));
    let rx_on = card.first(|o, v| o == REG_CR && v & HCI_RXDMA_EN != 0).expect("switched on");
    let rx_base = card.first(|o, v| o == REG_RXBD_DESA_MPDUQ && v == 0x3000_0000);
    let tx_base = card.first(|o, v| o == REG_TXBD_DESA_BEQ && v == 0x1000_0000);
    assert!(rx_base.expect("receive ring set") < rx_on, "receive ring first");
    assert!(tx_base.expect("transmit ring set") < rx_on, "transmit ring first");
}

// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs that the RTL8821CE `RtlLink` satisfies the shared `LinkPort` contract
//! net_core drives. An unassociated link reports down, offers no MAC and refuses
//! to transmit; once associated it reports its MAC and link up. A transmitted
//! Ethernet frame is framed by the station and laid into the TX ring with the
//! hardware CCMP security type set, so the radio encrypts from the CAM, and the
//! write-index doorbell is rung. A received 802.11 MPDU is pulled off the RX ring
//! and parsed back to the exact Ethernet frame that produced it. Driven through
//! the real `nonos_wifi_core::netif::LinkPort` trait against modeled DMA and a
//! modeled card; only the card actually moving bytes is left for on-silicon
//! bring-up.
//!
//! The staged receive frame is what this chip hands up (rtw88 reads the same
//! layout): the access point's FromDS frame, Protected bit still set, decrypted
//! in place with the CCMP header and MIC left on, the 4-byte FCS after it, and

//! A link over plain memory with a card that answers nothing, and a count of
//! the probes it laid on the transmit ring.

use std::cell::RefCell;
use std::rc::Rc;

use super::card::{Card, NoRx};
use crate::fw::dma::DmaMem;
use crate::link::RtlLink;

pub(super) const STA: [u8; 6] = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
pub(super) const AP: [u8; 6] = [0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

#[derive(Clone)]
pub(super) struct Dma(Rc<RefCell<Vec<u8>>>, u64);

impl DmaMem for Dma {
    fn capacity(&self) -> usize {
        self.0.borrow().len()
    }
    fn device_addr(&self) -> u64 {
        self.1
    }
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        self.0.borrow_mut()[offset..offset + src.len()].copy_from_slice(src);
    }
}

/// A link over fresh memory, and the transmit buffers it writes frames into.
pub(super) fn link() -> (RtlLink<Card, Dma, NoRx>, Dma) {
    let dma = |len: usize, at: u64| Dma(Rc::new(RefCell::new(vec![0u8; len])), at);
    let tx = dma(1 << 16, 0x2000_0000);
    (
        RtlLink::new(
            Card,
            dma(1 << 12, 0x1000_0000),
            tx.clone(),
            dma(1 << 12, 0x3000_0000),
            NoRx,
            STA,
        ),
        tx,
    )
}

/// How often `bytes` appear in the transmit buffers.
pub(super) fn count(tx: &Dma, bytes: &[u8]) -> usize {
    tx.0.borrow().windows(bytes.len()).filter(|w| *w == bytes).count()
}

/// The probe requests to `AP` in the transmit buffers.
pub(super) fn probes(tx: &Dma) -> usize {
    count(tx, &[&[0x40, 0, 0, 0][..], &AP, &STA, &AP].concat())
}

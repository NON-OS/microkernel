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

//! A card for a link that never receives: its registers read zero.

use crate::link::RxBuffers;
use crate::regs::Mmio;

/// Every register reads zero: no frame ever arrives, and the ring has room.
pub(super) struct Card;

impl Mmio for Card {
    fn read8(&self, _: usize) -> u8 {
        0
    }
    fn write8(&self, _: usize, _: u8) {}
    fn read16(&self, _: usize) -> u16 {
        0
    }
    fn write16(&self, _: usize, _: u16) {}
    fn read32(&self, _: usize) -> u32 {
        0
    }
    fn write32(&self, _: usize, _: u32) {}
}

pub(super) struct NoRx;

impl RxBuffers for NoRx {
    fn bytes(&self) -> &[u8] {
        &[]
    }
    fn device_addr(&self) -> u64 {
        0x3800_0000
    }
    fn clear(&mut self, _: usize, _: usize) {}
}

// NONOS Operating System (AGPL-3.0-or-later)

//! The rings' DMA regions, mapped through the broker below 4 GiB.

use super::addrs::{ACH0, CH12, CH8, RPQ, RXQ};
use super::rx::{self, RxRing};
use super::set::*;
use super::tx::{self, TxRing};
use crate::dma::{map_dma32, Grant};

/// Map and lay out every ring, or `None` when a region was refused.
pub fn map_rings(device_id: u64, epoch: u64) -> Option<Rings<Grant>> {
    let map = |bytes| map_dma32(device_id, epoch, bytes);
    let ach0 = TxRing::new(map(tx::bytes_for(ACH0_LEN))?, ACH0, ACH0_LEN)?;
    let ch8 = TxRing::new(map(tx::bytes_for(CH8_LEN))?, CH8, CH8_LEN)?;
    let ch12 = TxRing::new(map(tx::bytes_for(CH12_LEN))?, CH12, CH12_LEN)?;
    let idle = map(4096)?;
    let rxq = RxRing::new(map(rx::bytes_for(RXQ_LEN, RXQ_BUF))?, RXQ, RXQ_LEN, RXQ_BUF)?;
    let rpq = RxRing::new(map(rx::bytes_for(RPQ_LEN, RPQ_BUF))?, RPQ, RPQ_LEN, RPQ_BUF)?;
    Some(Rings { ach0, ch8, ch12, idle, rxq, rpq })
}

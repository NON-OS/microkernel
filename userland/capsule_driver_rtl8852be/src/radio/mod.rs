// NONOS Operating System (AGPL-3.0-or-later)

//! The chip as the shared station's soft-MAC radio (`nonos_wifi_core`'s
//! `Radio`): each call is the rtw89 path mac80211 would take for it.

mod counts;
mod frames;
mod keys;
mod learn;
mod link;
mod radio_impl;

pub use counts::Counts;

use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::sta::chip::Chip;
use alloc::vec::Vec;

pub struct Station<R: Mmio, M: DmaMem> {
    pub chip: Chip<R, M>,
    pub counts: Counts,
    pub(super) scratch: Vec<u8>,
}

impl<R: Mmio, M: DmaMem> Station<R, M> {
    pub fn new(chip: Chip<R, M>) -> Self {
        Station { chip, counts: Counts::default(), scratch: Vec::new() }
    }
}

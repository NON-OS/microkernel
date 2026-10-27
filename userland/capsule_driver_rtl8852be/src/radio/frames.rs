// NONOS Operating System (AGPL-3.0-or-later)

//! Frames in and out of the radio. RSSI comes in PPDU status reports,
//! which are not parsed: a frame's RSSI is 0, which the station reads as
//! "no reading".

use super::counts::bump;
use super::learn::from_frame;
use super::Station;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::rx::frame::Drop;
use crate::sta::pump::Got;
use crate::sta::send::NotSent;
use nonos_wifi_core::softmac::RxInfo;

impl<R: Mmio, M: DmaMem> Station<R, M> {
    /// `send`: release reports taken first, so their slots are free.
    pub fn push(&mut self, mpdu: &[u8]) -> bool {
        self.chip.reap(&mut self.scratch);
        match self.chip.send(mpdu) {
            Ok(()) => true,
            Err(NotSent::Ring(_)) => {
                bump(&mut self.counts.tx_full);
                false
            }
            Err(_) => false,
        }
    }

    /// `recv`: the next frame, any buffer that was not one counted.
    pub fn pull(&mut self, out: &mut [u8]) -> Option<RxInfo> {
        self.chip.reap(&mut self.scratch);
        loop {
            match self.chip.next_rx(&mut self.scratch, out)? {
                Got::Frame(len, decrypted) => {
                    from_frame(&mut self.chip.vif, &out[..len]);
                    return Some(RxInfo { len, rssi_dbm: 0, decrypted });
                }
                Got::Dropped(Drop::BadCrc) => bump(&mut self.counts.rx_bad_crc),
                Got::Dropped(_) => bump(&mut self.counts.rx_drop),
            }
        }
    }
}

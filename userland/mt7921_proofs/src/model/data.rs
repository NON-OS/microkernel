// NONOS Operating System (AGPL-3.0-or-later)

//! Frames through the model. A data-ring slot is a TXD and TXP; the frame is
//! read where the TXP points, kept, sealed if the TXD says (`crypt`), put on
//! the air, its token named in a transmit-free report on ring 2. A frame
//! heard is opened if it can be (`crypt`) and put on ring 2 behind its RXD
//! (`rxd`). A frame sent at a rate the AP does not decode is not heard.

use super::chip::Chip;
use super::crypt::Heard;
use super::mcu_cmds::le32;
use super::rxd::rx_buffer_with;
use super::{mem::read_mem, phy};

/// One frame the driver sent: its TXD words, the frame, and what went on
/// the air (`None`: the chip had no key to seal it under).
#[derive(Clone, Debug)]
pub struct TxFrame {
    pub txd: [u32; 8],
    pub token: u16,
    pub frame: Vec<u8>,
    pub air: Option<Vec<u8>>,
}

impl TxFrame {
    pub fn wcid(&self) -> u32 {
        self.txd[1] & 0x3ff
    }

    pub fn rate(&self) -> u32 {
        self.txd[6] >> 16 & 0x3fff
    }
}

impl Chip {
    pub(super) fn transmit(&mut self, txwi: &[u8]) {
        if txwi.len() < 64 {
            return;
        }
        let txd: [u32; 8] = core::array::from_fn(|i| le32(txwi, 4 * i));
        let token = u16::from_le_bytes([txwi[32], txwi[33]]) & 0x7fff;
        let len = (u16::from_le_bytes([txwi[44], txwi[45]]) & 0xfff) as usize;
        let frame = read_mem(le32(txwi, 40), len).unwrap_or_default();
        let air =
            if txd[3] & 2 != 0 { self.seal(txd[1] & 0x3ff, &frame) } else { Some(frame.clone()) };
        self.sent.push(TxFrame { txd, token, frame, air: air.clone() });
        if !self.faults.no_txfree {
            let w0 = 12 | 6 << 27 | 1 << 16;
            let report = [w0, 0, u32::from(token) << 16];
            self.push_rx(2, report.iter().flat_map(|w| w.to_le_bytes()).collect());
        }
        let (tuned, Some(frame)) = (self.mcu.tuned(), air) else { return };
        if !phy::heard(txd[6], tuned, self.g_only) {
            self.unheard += 1;
            return;
        }
        let mut replies = self.air.as_mut().map(|a| a.on_frame(&frame, tuned)).unwrap_or_default();
        replies.extend(self.wpa3.as_mut().map(|a| a.on_frame(&frame, tuned)).unwrap_or_default());
        for f in replies {
            self.receive(f);
        }
    }

    /// A frame heard on the air, onto ring 2.
    pub fn receive(&mut self, frame: Vec<u8>) {
        let frame = if self.g_only { phy::g_only(frame) } else { frame };
        let rssi = self.air.as_ref().map_or(-50, |a| a.rssi);
        let b = match self.open(&frame) {
            Heard::Plain => rx_buffer_with(&frame, rssi, 0, 0),
            Heard::Opened(d1, pn, f) => rx_buffer_with(&f, rssi, d1, pn),
            Heard::Dropped => return,
        };
        self.push_rx(2, b);
    }
}

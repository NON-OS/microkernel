// NONOS Operating System (AGPL-3.0-or-later)

//! Frames through the model. A data-ring slot is a TXD and TXP; the frame
//! is read where the TXP points, kept, put on the air, and its token named
//! in a transmit-free report on ring 2 (`PKT_TYPE_TXRX_NOTIFY`, one MSDU).
//! A frame heard is written to ring 2 behind a normal RXD with group 3,
//! whose first chain carries the signal as an RCPI. A frame sent at a rate
//! the access point does not decode is not heard (`phy`).

use super::chip::Chip;
use super::mcu_cmds::le32;
use super::{mem::read_mem, phy};

/// One frame the driver sent: its TXD words and the frame.
#[derive(Clone, Debug)]
pub struct TxFrame {
    pub txd: [u32; 8],
    pub token: u16,
    pub frame: Vec<u8>,
}

impl TxFrame {
    pub fn wcid(&self) -> u32 {
        self.txd[1] & 0x3ff
    }

    pub fn rate(&self) -> u32 {
        self.txd[6] >> 16 & 0x3fff
    }
}

/// A received frame as the chip writes it: RXD, group 3, the frame.
pub fn rx_buffer(frame: &[u8], rssi: i8) -> Vec<u8> {
    let rcpi = (2 * i32::from(rssi) + 220) as u32 & 0xff;
    let len = (32 + frame.len()) as u32;
    let words = [len | 2 << 27, 1 << 13, 0, 0, 0, 0, 0, rcpi | rcpi << 8];
    let mut out: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    out.extend_from_slice(frame);
    out
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
        self.sent.push(TxFrame { txd, token, frame: frame.clone() });
        if !self.faults.no_txfree {
            let w0 = 12 | 6 << 27 | 1 << 16;
            let report = [w0, 0, u32::from(token) << 16];
            self.push_rx(2, report.iter().flat_map(|w| w.to_le_bytes()).collect());
        }
        let tuned = self.mcu.tuned();
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
        self.push_rx(2, rx_buffer(&frame, rssi));
    }
}

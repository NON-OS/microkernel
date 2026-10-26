// NONOS Operating System (AGPL-3.0-or-later)

//! The parsed RX descriptor.

use super::super::regs::RX_PKT_DESC_SIZE;

/// The parsed RX descriptor: where the frame is and whether it is usable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RxInfo {
    pub pkt_len: usize,
    pub drv_info_size: usize,
    pub shift: usize,
    pub phy_status: bool,
    pub crc_err: bool,
    pub icv_err: bool,
    pub is_c2h: bool,
    /// The security type the chip saw (`RX_DESC_ENC_*`; 0 none, 4 AES).
    pub enc_type: u8,
    /// The chip left decryption to software.
    pub swdec: bool,
}

impl RxInfo {
    /// Offset from the start of the descriptor to the 802.11 frame.
    pub fn frame_offset(&self) -> usize {
        RX_PKT_DESC_SIZE + self.drv_info_size + self.shift
    }
    /// Total bytes this descriptor plus its frame occupy in the buffer.
    pub fn total_len(&self) -> usize {
        self.frame_offset() + self.pkt_len
    }
    /// A frame worth delivering: a valid 802.11 frame, not a firmware command or
    /// an error frame.
    pub fn deliverable(&self) -> bool {
        !self.is_c2h && !self.crc_err && !self.icv_err && self.pkt_len != 0
    }
    /// The chip decrypted (and verified) a protected frame. It leaves the
    /// CCMP header and MIC in place.
    pub fn decrypted(&self) -> bool {
        !self.swdec && self.enc_type != 0
    }
}

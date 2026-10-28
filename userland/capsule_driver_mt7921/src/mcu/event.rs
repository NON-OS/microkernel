/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! What the MCU writes into an event ring: the hardware RXD (six words),
//! then `struct mt76_connac2_mcu_rxd`'s header, then the body at byte 36.
//! Sorted as Linux v6.12 `mt7921_mcu_rx_event` sorts: unsolicited events,
//! and replies, which carry the command's sequence number.

/// The body's start (`sizeof(struct mt76_connac2_mcu_rxd)`).
pub const BODY: usize = 36;
/// `PKT_TYPE_RX_EVENT`, RXD0 bits 31:27.
pub const PKT_TYPE_RX_EVENT: u32 = 7;
/// `MCU_UNI_CMD_UNSOLICITED_EVENT` in the header's option byte.
const OPT_UNSOLICITED: u8 = 1 << 2;
/// Events Linux handles as unsolicited however their sequence reads:
/// beacon loss, scheduled scan done, RSSI, scan done, TX done, debug
/// message, core dump, low power.
const UNSOLICITED: [u8; 8] = [0x13, 0x23, 0x96, 0x0d, 0x0f, 0x27, 0xf0, 0x07];
/// `MCU_EXT_EVENT_RATE_REPORT`.
const EXT_RATE_REPORT: u8 = 0x87;
/// The generic event (0x6) is always a reply.
const EID_REPLY: u8 = 0x6;

#[derive(Clone, Copy)]
pub struct Event<'a>(pub &'a [u8]);

impl<'a> Event<'a> {
    /// An MCU event, or `None` for anything else or a buffer too short to
    /// hold its header. Its extent is what the DMA wrote, as Linux takes
    /// it; the RXD's own length field is not relied on.
    pub fn parse(b: &'a [u8]) -> Option<Event<'a>> {
        let dw0 = u32::from_le_bytes(b.get(..4)?.try_into().ok()?);
        (dw0 >> 27 == PKT_TYPE_RX_EVENT && b.len() >= BODY).then_some(Event(b))
    }

    pub fn eid(&self) -> u8 {
        self.0[28]
    }

    pub fn seq(&self) -> u8 {
        self.0[29]
    }

    pub fn ext_eid(&self) -> u8 {
        self.0[32]
    }

    pub fn body(&self) -> &'a [u8] {
        &self.0[BODY..]
    }

    /// Whether the event answers no command.
    pub fn unsolicited(&self) -> bool {
        if self.0[30] & OPT_UNSOLICITED != 0 {
            return true;
        }
        if self.eid() == EID_REPLY {
            return false;
        }
        let e = self.eid();
        self.ext_eid() == EXT_RATE_REPORT || UNSOLICITED.contains(&e) || self.seq() == 0
    }

    /// Whether this is the unified event `eid` sent unasked (the ROC grant
    /// is `MCU_UNI_EVENT_ROC`, 0x27).
    pub fn uni_unsolicited(&self, eid: u8) -> bool {
        self.0[30] & OPT_UNSOLICITED != 0 && self.eid() == eid
    }
}

/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Talking to the chip's MCU: commands out on ring 17 (ring 16 for the
//! firmware's bytes), replies and unasked events in on receive rings 0 and
//! 4, matched by sequence number as `mt76_mcu_skb_send_and_get_msg` and
//! `mt7921_mcu_parse_response` match them. Polled: a reply is waited for
//! by reading the rings, never by an interrupt.

pub mod cmd;
pub mod event;
pub mod frame;
mod pump;
mod reply;
mod send;

pub use reply::Reply;

/// Why a command did not go through.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McuError {
    /// The ring stayed full: the chip is not reading commands.
    NoRoom,
    /// The body does not fit a command slot.
    TooLong,
    /// No reply within the wait (Linux's is three seconds).
    Timeout,
    /// The reply's own outcome was not success.
    Status(u32),
    /// The card's firmware held it and would not give it back.
    Asleep,
}

/// The last ROC grant the MCU sent: its token, and the channel it names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Grant {
    pub token: u8,
    pub channel: u8,
}

#[derive(Default)]
pub struct Mcu {
    seq: u8,
    /// The grant most recently heard, until the next request clears it.
    pub grant: Option<Grant>,
    /// Unasked events seen.
    pub events: u32,
}

impl Mcu {
    /// `++msg_seq & 0xf`, never zero: zero marks an unasked event.
    fn next_seq(&mut self) -> u8 {
        self.seq = self.seq.wrapping_add(1) & 0xf;
        if self.seq == 0 {
            self.seq = 1;
        }
        self.seq
    }
}

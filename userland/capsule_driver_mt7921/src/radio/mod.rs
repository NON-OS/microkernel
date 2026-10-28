/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The card as the shared station's radio (`nonos_wifi_core::softmac::
//! Radio`): the station builds and seals every frame and runs the scan,
//! the join, WPA2 and WPA3 and the data path; the card tunes by remain-on-
//! channel grants, carries frames on rings 0 and 2, holds the link's keys
//! and runs CCMP-128 on them, and is told of the access point and the
//! association the way mac80211 tells Linux's driver.

mod assoc;
mod bss;
mod data;
mod diag;
mod keys;
mod learn;
mod port;
mod recv;
mod roc;
mod send;
mod tune;

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use nonos_wifi_core::regdom::Band;

use crate::card::Card;
use crate::ids::Chip;
use crate::init::caps::Caps;
use crate::vif::bss::Basic;
use crate::vif::sta::Peer;
pub use roc::Held;

pub struct Mt7921 {
    pub card: Card,
    pub chip: Chip,
    pub caps: Caps,
    pub mac: [u8; 6],
    /// The channel the radio was last granted.
    pub at: Option<(Band, u8)>,
    /// The associated BSS's channel, which its context holds.
    pub home: Option<(Band, u8)>,
    pub roc: Option<Held>,
    pub token: u8,
    pub peer: Option<Peer>,
    /// What association settled: the BSS as told, its basic rates.
    pub joined: Option<(Basic, (u8, u8))>,
    /// The access points heard, by their basic rates.
    pub aps: learn::Heard,
    /// Frames read while making room for a send, delivered first, with
    /// their signal and whether the chip decrypted them.
    pub inbox: VecDeque<(Vec<u8>, i8, bool)>,
    pub count: diag::Counts,
    /// The card stopped answering: nothing is sent to it again.
    pub dead: bool,
}

impl Mt7921 {
    pub fn new(card: Card, chip: Chip, caps: Caps, mac: [u8; 6]) -> Self {
        let (at, home, roc, peer, joined) = (None, None, None, None, None);
        let (inbox, count, token, dead) = (VecDeque::new(), diag::Counts::default(), 0, false);
        let aps = learn::Heard::default();
        Self { card, chip, caps, mac, at, home, roc, token, peer, joined, aps, inbox, count, dead }
    }

    pub(crate) fn ghz5(&self) -> bool {
        matches!(self.at, Some((Band::Ghz5, _)))
    }
}

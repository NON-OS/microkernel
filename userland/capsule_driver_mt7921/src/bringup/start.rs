/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The firmware downloaded and the MAC brought up, tried up to three
//! times with Linux's forced reset between (`mt7921_init_hardware` tries
//! ten; three keeps a failing card from holding the panel's answer for
//! minutes); then the interface added under a random address and the
//! receive filter set to hear every network.

use super::package::Files;
use super::rings::{quiet, rewind};
use crate::card::Card;
use crate::ids::Chip;
use crate::init::{self, caps::Caps};
use crate::radio::Mt7921;
use crate::say::{line, Line};
use crate::setup::Mapped;
use crate::stage::Stage;
use crate::vif;

const ATTEMPTS: usize = 3;

pub fn firmware(card: &mut Card, files: &Files) -> Result<Caps, Stage> {
    for attempt in 1..=ATTEMPTS {
        let c = &mut *card;
        match crate::fw::download(&mut c.mcu, &c.regs, &mut c.dma, &files.patch, &files.ram) {
            Ok(()) => {
                line("stage firmware: patch and RAM code running");
                match init::run(&mut c.mcu, &c.regs, &mut c.dma) {
                    Ok(caps) => {
                        let l = Line::new("stage mac: up, bands ");
                        l.dec(caps.ghz2 as u64).text(",").dec(caps.ghz5 as u64).say();
                        return Ok(caps);
                    }
                    Err(s) => Line::new("mac init failed at step ").dec(s as u64).say(),
                }
            }
            Err(s) => Line::new("firmware failed at step ").dec(s as u64).say(),
        }
        Line::new("attempt ").dec(attempt as u64).text(" failed; resetting").say();
        rewind(card);
    }
    Err(Stage::FirmwareFailed)
}

/// A locally administered unicast address from the kernel's randomness,
/// which the station replaces per network before anything is sent; the
/// card's factory address is never read.
fn draw() -> Option<[u8; 6]> {
    let mut mac = [0u8; 6];
    let n = nonos_libc::crypto_random(mac.as_mut_ptr(), mac.len());
    (n == mac.len() as i64).then(|| {
        nonos_mac::apply(&mut mac);
        mac
    })
}

/// The interface added; on a failure the card is left quiet.
pub fn interface(mut card: Card, m: &Mapped, chip: Chip, caps: Caps) -> Result<Mt7921, Stage> {
    let Some(mac) = draw() else {
        quiet(&card, m);
        return Err(Stage::NoStationAddress);
    };
    if !vif::add(&mut card, mac) || !vif::listen(&mut card) {
        line("interface: the firmware refused the station's entry");
        quiet(&card, m);
        return Err(Stage::FirmwareFailed);
    }
    line("stage interface: station added");
    Ok(Mt7921::new(card, chip, caps, mac))
}

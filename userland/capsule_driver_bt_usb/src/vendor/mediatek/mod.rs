/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! MediaTek MT7921 and MT7922 Bluetooth over USB: a controller that is
//! no Bluetooth controller until its ROM is patched. Before HCI bring-up,
//! a MediaTek controller (`ids`) is asked which chip it is (`chip`), its
//! patch is read from the firmware package and held to its pin (`files`,
//! `pins`, `fetch`), the chip is reset (`reset`), the patch is sent
//! section by section over WMT (`image`, `load`, `wmt`) and the Bluetooth
//! function is turned on (`setup`). Every file but this and `glue` runs in
//! `bt_usb_proofs` against a modelled controller.

mod chip;
mod fail;
mod fetch;
mod files;
mod glue;
mod ids;
mod image;
mod load;
mod pins;
mod port;
mod reg;
mod reset;
mod setup;
mod wmt;

use glue::{digits, say, Glue};

use nonos_usbnet::Found;

use crate::usb::Bound;

/// The controller bound from `found`, once it is ready for HCI: one that
/// is not MediaTek's as it was, a MediaTek one once patched.
pub fn up(mut b: Bound, found: &Found) -> Option<Bound> {
    if !ids::ours(found.info.vendor, found.info.product) {
        return Some(b);
    }
    match setup::setup(&mut Glue(&mut b.pipes), &files::FILES) {
        Ok(up) => {
            let (sent, held) = (digits(up.sent.sent), digits(up.sent.applied));
            let chip = up.chip.name();
            say(&[b"MediaTek ", chip, b" patched: ", &sent, b" sections sent, ", &held, b" held"]);
            Some(b)
        }
        Err(f) => {
            say(&[b"MediaTek controller not set up: ", f.words()]);
            None
        }
    }
}

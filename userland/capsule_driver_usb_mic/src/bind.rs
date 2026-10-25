// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! Whether a device has a microphone this driver serves, and setting it up:
//! its configuration selected and its capture interface left at alternate
//! 0, so it takes no bandwidth until a stream starts. A headset's playback
//! side is driver.usb_audio0's on its own slot; a device both ways is
//! taken by whichever driver claims its port first.

use alloc::vec::Vec;

use nonos_usbnet::bus::Bus;
use nonos_usbnet::setup::Setup;
use nonos_usbnet::xhci::{XhciBus, E_INVAL};
use nonos_usbnet::{Bind, Found};

use crate::uac::{find, timing, Stream, Timing, SAMPLE_BYTES};
use crate::xhci::{port_speed, BATCH_BYTES};

pub struct Mic {
    pub s: Stream,
    pub timing: Timing,
    /// Its configuration descriptor: a 2.0 clock is looked up in it.
    pub config: Vec<u8>,
    pub ids: (u16, u16),
}

pub fn bind(mut bus: XhciBus, found: &Found) -> Bind<Mic> {
    let chosen = found.configs.iter().find_map(|c| Some((c, c.get(5).copied()?, find(c, false)?)));
    let Some((raw, value, s)) = chosen else { return Bind::NotOurs };
    let speed = match port_speed(bus.xhci, found.port) {
        Ok(sp) => sp,
        Err(e) => return Bind::Failed("port speed unread", e),
    };
    let Some(timing) = timing(speed, s.b_interval, s.max_packet, s.channels) else {
        return Bind::Failed("no 48 kHz packet fits its endpoint", E_INVAL);
    };
    // An asynchronous device sends a frame more now and then to keep up
    // with its own clock: a batch must hold one more a packet.
    let frame = SAMPLE_BYTES as usize * s.channels as usize;
    if timing.batch_bytes() + timing.packets as usize * frame > BATCH_BYTES {
        return Bind::Failed("a batch would not fit one reply", E_INVAL);
    }
    if let Err(e) = bus.control_out(Setup::set_configuration(value), &[]) {
        return Bind::Failed("set configuration", e);
    }
    if let Err(e) = bus.control_out(Setup::set_interface(s.interface, 0), &[]) {
        return Bind::Failed("set interface 0", e);
    }
    let ids = (found.info.vendor, found.info.product);
    Bind::Ours(Mic { s, timing, config: raw.clone(), ids })
}

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

//! The device's unasked messages: its notification endpoint looked at once
//! every LOOK_EVERY receives, and when it says a response waits, that
//! response read once and acted on: a media state kept, a KEEPALIVE
//! answered. Without the endpoint the link reads up while bound.

use nonos_usbnet::xhci::CONTROL_MAX;
use nonos_usbnet::{Bus, Setup};

use super::from_device::{from_device, keepalive_done, FromDevice};
use super::link::Rndis;
use super::message::{GET_ENCAPSULATED_RESPONSE, SEND_ENCAPSULATED_COMMAND};

const LOOK_EVERY: u32 = 64;

pub(super) fn look_at_link<B: Bus>(nic: &mut Rndis<B>) {
    nic.looks = nic.looks.wrapping_add(1);
    let Some(dci) = nic.notify.filter(|_| nic.looks.is_multiple_of(LOOK_EVERY)) else { return };
    let mut n = [0u8; 16];
    if !matches!(nic.bus.interrupt_in_poll(dci, &mut n), Ok(Some(_))) {
        return;
    }
    let mut r = [0u8; CONTROL_MAX];
    let get = Setup::class(true, GET_ENCAPSULATED_RESPONSE, 0, nic.comm);
    let Ok(len) = nic.bus.control_in(get, &mut r) else { return };
    match from_device(&r[..len.min(CONTROL_MAX)]) {
        FromDevice::Media(up) => nic.link = up,
        FromDevice::KeepAlive(id) => {
            let send = Setup::class(false, SEND_ENCAPSULATED_COMMAND, 0, nic.comm);
            let _ = nic.bus.control_out(send, &keepalive_done(id));
        }
        FromDevice::Other => {}
    }
}

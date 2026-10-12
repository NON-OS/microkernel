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

//! The words the page's rows read, from what the service last said.

use nonos_bt_proto::ops::{CONNECTING, IDLE, NO_CONTROLLER, READY, SCANNING, SECURING};
use nonos_bt_proto::{Link, VERSION};

use crate::settings::schema::rows::Tone;

use super::known::Panel;
use super::notice::unreached;
use super::reach::Reach;

/// The adapter: whether driver.bt_usb0 found a controller it brought up.
pub fn controller(p: &Panel) -> (&'static str, Tone) {
    match (p.reach, p.health) {
        (Reach::Unasked, _) => ("--", Tone::Idle),
        (Reach::Answered, Some(h)) if h.version != VERSION => ("Speaks another NBTC", Tone::Warn),
        (Reach::Answered, Some(h)) if h.controller => ("USB adapter ready", Tone::Ok),
        (Reach::Answered, _) => ("No USB adapter found", Tone::Warn),
        (reach, _) => (unreached(reach), Tone::Warn),
    }
}

/// The link, as LINK's state and flags give it.
pub fn status(p: &Panel) -> (&'static str, Tone) {
    let Some(l) = p.link.filter(|_| p.reach == Reach::Answered) else { return controller(p) };
    let device = (l.has(Link::KEYBOARD), l.has(Link::MOUSE));
    match l.state {
        NO_CONTROLLER => ("No USB adapter found", Tone::Warn),
        IDLE => ("Not looking for devices", Tone::Idle),
        SCANNING => ("Looking for devices", Tone::Idle),
        CONNECTING => ("Connecting", Tone::Idle),
        SECURING => ("Securing the link", Tone::Idle),
        READY => match device {
            (true, true) => ("Keyboard and mouse connected", Tone::Ok),
            (true, false) => ("Keyboard connected", Tone::Ok),
            (false, true) => ("Mouse connected", Tone::Ok),
            (false, false) => ("Connected", Tone::Ok),
        },
        _ => ("--", Tone::Idle),
    }
}

pub fn scanning(p: &Panel) -> (&'static str, Tone) {
    match p.link {
        Some(l) if p.reach == Reach::Answered && l.has(Link::SCANNING) => ("On", Tone::Ok),
        Some(_) if p.reach == Reach::Answered => ("Off", Tone::Idle),
        _ => ("--", Tone::Idle),
    }
}

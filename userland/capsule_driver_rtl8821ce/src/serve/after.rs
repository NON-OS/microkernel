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

//! What the loop does besides answering: what a link request left, and a
//! step of the background scan.

use nonos_wifi_core::key::KeyStore;

use super::connect::{self, Session};
use super::radio::{self, Radio};
use super::scanner::Scanner;

// What the receive path left for the loop: a group key the AP rekeyed to goes
// into the CAM (the old index is cleared once the new one is in), and a
// deauthentication from the AP ends the session, clearing its keys, so the
// link reads down and the background scan resumes.
pub(super) fn after_receive(up: &mut radio::RadioUp, session: &mut Option<Session>) {
    if let (Some((id, key)), Some(s)) = (up.link.take_group_key(), session.as_mut()) {
        if up.keys.install_gtk(&key, id) {
            s.set_gtk_id(id);
        }
    }
    if up.link.take_left().is_some() {
        connect::disconnect(&mut up.link, &mut up.keys, &up.regs, session);
    }
}

// Advance the background scan while up and not associated. Draining runs on
// every pass so net_core traffic cannot starve it, and the channel hops on
// the clock, so a busy pass moves on as surely as an idle one. A session
// stops the scan, so it never knocks a live connection off its channel.
pub(super) fn scan_step(radio: &mut Radio, session: &Option<Session>, scanner: &mut Scanner) {
    if session.is_none() {
        if let Radio::Up(up) = radio {
            scanner.drain(&mut up.link, &up.regs);
            scanner.advance(&up.regs);
        }
    }
}

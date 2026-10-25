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

//! What the loop does besides answering: what a link request left, the
//! link's clock, and a step of the background scan.

use nonos_wifi_core::key::KeyStore;

use super::connect::{self, reseed, Session};
use super::loss::say_loss;
use super::radio::{self, Radio};
use super::scanner::Scanner;

// What the receive path left for the loop: a group key the AP rekeyed to goes
// into the CAM (the old index is cleared once the new one is in), and a
// deauthentication from the AP ends the session, clearing its keys, so the
// link reads down and the background scan resumes.
pub(super) fn after_receive(up: &mut radio::RadioUp, session: &mut Option<Session>) {
    // A rekeyed pairwise key replaces the CAM's in its slot.
    if let (Some(key), Some(s)) = (up.link.pairwise_key(), session.as_ref()) {
        up.keys.install_ptk(key, s.key_id, &s.bssid);
        reseed(&mut up.link);
    }
    up.link.clear_pairwise_key();
    if let (Some((id, key)), Some(s)) = (up.link.group_key(), session.as_mut()) {
        if up.keys.install_gtk(key, id) {
            s.set_gtk_id(id);
        }
    }
    up.link.clear_group_key();
    let (left, silent) = (up.link.take_left(), up.link.take_silent());
    if left.is_some() || silent {
        say_loss(left);
        connect::disconnect(&mut up.link, &mut up.keys, &up.regs, session);
    }
}

// Each pass: tick the link of a session (its probes name the session's
// network), close the session if the link ended, then step the scan.
pub(super) fn after_pass(radio: &mut Radio, session: &mut Option<Session>, scanner: &mut Scanner) {
    if let (Radio::Up(up), Some(s)) = (&mut *radio, session.as_ref()) {
        up.link.set_ssid(&s.ssid[..usize::from(s.ssid_len).min(s.ssid.len())]);
        up.link.tick(u64::try_from(nonos_libc::mk_uptime_ms()).unwrap_or(0));
        after_receive(up, session);
    }
    scan_step(radio, session, scanner);
}

// Advance the background scan while up and not associated. Draining runs on
// every pass so net_core traffic cannot starve it, and the channel hops on
// the clock, so a busy pass moves on as surely as an idle one. A session
// stops the scan, so it never knocks a live connection off its channel.
fn scan_step(radio: &mut Radio, session: &Option<Session>, scanner: &mut Scanner) {
    if session.is_none() {
        if let Radio::Up(up) = radio {
            scanner.drain(&mut up.link, &up.regs);
            scanner.advance(&up.regs);
        }
    }
}

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
//! The link, looked at once autojoin is done: the network it is up on is kept
//! (a name, no secret), so a rejoin after it goes down asks for that one alone.

use spin::Mutex;

use nonos_wifi_client::{find, SSID_MAX};

use super::machine::{Machine, CALL_MS};
use super::tick::say;

/// The SSID the link was last seen up on, and its length.
static UP_ON: Mutex<([u8; SSID_MAX], usize)> = Mutex::new(([0; SSID_MAX], 0));

/// Look at the link and tell the machine; say so when a rejoin starts.
pub(super) fn look(m: &mut Machine, now: i64) {
    let link = find().and_then(|d| d.link_within(CALL_MS));
    if let Some(l) = link.as_ref().filter(|l| l.associated) {
        let ssid = l.ssid();
        let mut up = UP_ON.lock();
        let len = ssid.len().min(SSID_MAX);
        up.0[..len].copy_from_slice(&ssid[..len]);
        up.1 = len;
    }
    if m.after_link(now, link.map(|l| l.associated)) {
        say(b"[NET-CORE] Wi-Fi link lost; joining the network again\n");
    }
}

/// The SSID the link was last up on.
pub(super) fn up_on() -> ([u8; SSID_MAX], usize) {
    *UP_ON.lock()
}

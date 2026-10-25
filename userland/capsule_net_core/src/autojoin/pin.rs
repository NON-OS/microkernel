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
//! The WPA3 pin: a network the link is up on through SAE, or whose access
//! point set Transition Disable in message 3, is pinned to WPA3 in the saved
//! list, once per association. Only the person's Forget clears a pin.

use spin::Mutex;

use nonos_wifi_client::{pin_wpa3, Link, SSID_MAX};

use super::tick::say;

/// The SSID this association was pinned on (or tried), and its length; empty
/// once the link is seen down.
static PINNED_ON: Mutex<([u8; SSID_MAX], usize)> = Mutex::new(([0; SSID_MAX], 0));

/// After a look at the link: pin the network it is up on when it asks for
/// it. A network not saved stays unsaved; a failed write is not retried
/// until the next association, so a boot that keeps nothing is not asked
/// again on every look.
pub(super) fn after_look(link: Option<&Link>) {
    let Some(l) = link else {
        return;
    };
    let mut on = PINNED_ON.lock();
    if !l.associated {
        on.1 = 0;
        return;
    }
    let ssid = l.ssid();
    if !l.pins_wpa3() || &on.0[..on.1] == ssid {
        return;
    }
    let len = ssid.len().min(SSID_MAX);
    on.0[..len].copy_from_slice(&ssid[..len]);
    on.1 = len;
    drop(on);
    if pin_wpa3(ssid) == Ok(true) {
        say(b"[NET-CORE] Wi-Fi network pinned to WPA3\n");
    }
}

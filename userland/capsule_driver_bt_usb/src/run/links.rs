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

//! Links one after another on a controller that is up: renew this host's
//! address if due (between scans, the one time it may change: Vol 4,
//! Part E, 7.8.4), look for a HID device, connect, pair or re-encrypt,
//! serve it until the link ends, and look again. Only a failing controller
//! ends the loop.

use nonos_bt_hci::host::{Connect, DEFAULT_LINK, DEFAULT_SCAN};
use nonos_bt_hci::usb::UsbHci;
use nonos_bt_hci::{HciError, Host};

use super::ended::ended;
use super::env::CapsuleEnv;
use super::say::say;
use crate::link::{begin, find_hid, serve, End, Private};
use crate::usb::XhciPipes;

pub fn links(
    host: &mut Host<UsbHci<XhciPipes>>,
    mut us: Private,
    env: &mut CapsuleEnv,
) -> HciError {
    loop {
        let me = match us.refresh(host, env) {
            Ok(me) => me,
            Err(e) => return e,
        };
        let peer = match find_hid(host, env) {
            Ok(Some(peer)) => peer,
            Ok(None) => continue,
            Err(e) => return e,
        };
        let (peer_type, peer) = (peer.kind, peer.addr);
        let c = Connect { scan: DEFAULT_SCAN, peer_type, peer, link: DEFAULT_LINK };
        let link = match host.connect(&c) {
            Ok(link) => link,
            Err(HciError::Transport(e)) => return HciError::Transport(e),
            // It stopped advertising, or the controller would not: look again.
            Err(_) => continue,
        };
        say(&[b"connected"]);
        let end = begin(host, &link, &me, env).and_then(|s| match s {
            Some(mut s) => serve(host, &mut s, env),
            None => Ok(End::GaveUp("pairing could not start")),
        });
        if let Err(e) = ended(host, end, link.handle) {
            return e;
        }
    }
}

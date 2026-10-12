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
//! address if due (between scans, the one time it may change: Vol 4, Part
//! E, 7.8.4), look for a HID device while scanning is on, connect, pair or
//! re-encrypt, serve it until it ends, and again; the board follows along.

use nonos_bt_hci::host::{Connect, DEFAULT_LINK, DEFAULT_SCAN};
use nonos_bt_hci::{usb::UsbHci, HciError, Host};

use super::ended::ended;
use super::env::CapsuleEnv;
use super::say::say;
use crate::link::{begin, find_hid, serve, End, Private};
use crate::nbtc::ops::{CONNECTING, IDLE, SCANNING, SECURING};
use crate::usb::XhciPipes;

/// How long a look waits while scanning is off, answering NBTC meanwhile.
const IDLE_MS: u64 = 250;

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
        let scan = env.desk.board().scan;
        env.desk.board().state = if scan { SCANNING } else { IDLE };
        if !scan {
            env.desk.wait(IDLE_MS);
            continue;
        }
        let peer = match find_hid(host, env) {
            Ok(Some(peer)) => peer,
            Ok(None) => continue,
            Err(e) => return e,
        };
        env.desk.board().state = CONNECTING;
        let (peer_type, peer, scan, link) = (peer.kind, peer.addr, DEFAULT_SCAN, DEFAULT_LINK);
        let c = Connect { scan, peer_type, peer, link };
        let link = match host.connect(&c) {
            Ok(link) => link,
            Err(HciError::Transport(e)) => return HciError::Transport(e),
            // It stopped advertising, or the controller would not: look again.
            Err(_) => continue,
        };
        say(&[b"connected"]);
        env.desk.board().state = SECURING;
        let end = begin(host, &link, &me, env).and_then(|s| match s {
            Some(mut s) => serve(host, &mut s, env),
            None => Ok(End::GaveUp("pairing could not start")),
        });
        if let Err(e) = ended(host, end, link.handle, &env.desk) {
            return e;
        }
    }
}

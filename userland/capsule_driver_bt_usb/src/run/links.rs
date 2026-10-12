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

//! Links one after another on a controller that is up: look for a HID
//! device, connect, pair, serve it until the link ends, and look again.
//! Only a failing controller ends the loop; a device that will not pair
//! is left alone for a while, so it is not asked again and again.

use nonos_bt_hci::host::{Connect, DEFAULT_LINK, DEFAULT_SCAN};
use nonos_bt_hci::usb::UsbHci;
use nonos_bt_hci::{HciError, Host};
use nonos_bt_wire::addr::BdAddr;

use super::env::CapsuleEnv;
use super::say::say;
use super::wait::wait_ms;
use crate::link::{find_hid, hang_up, serve, End, Session};
use crate::usb::XhciPipes;

/// After a link this host gave up on, before the next look.
const BACK_OFF_MS: u64 = 30_000;

pub fn links(host: &mut Host<UsbHci<XhciPipes>>, ours: &BdAddr, env: &mut CapsuleEnv) -> HciError {
    loop {
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
        let end = match Session::open(&link, ours, env) {
            Ok(mut s) => serve(host, &mut s, env),
            Err(_) => Ok(End::GaveUp("pairing could not start")),
        };
        match end {
            Ok(End::Lost(_)) => say(&[b"link lost"]),
            Ok(End::GaveUp(why)) => {
                say(&[b"link ended: ", why.as_bytes()]);
                if let Err(e) = hang_up(host, link.handle) {
                    return e;
                }
                wait_ms(BACK_OFF_MS);
            }
            Err(e) => return e,
        }
    }
}

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

//! One request, answered. net_core's link operations come first, and only
//! to net.core's owner (nonos_wifi_core::netif::gate): the link puts raw
//! frames on the air, below the route the person chose. The Wi-Fi control
//! family is the fallthrough, for the panels. A frame neither takes is
//! refused, so its caller is answered.

use nonos_wifi_core::netif::{self, is_link, link_admits, refuse_link};

use crate::link::DeadLink;

use super::after::after_receive;
use super::connect::Session;
use super::control::control;
use super::net_core::net_core_pid;
use super::radio::Radio;
use super::refuse::refuse;
use super::scanner::Scanner;
use super::Stage;

/// The reply's length in `tx`.
pub(super) fn answer(
    req: &mut [u8],
    sender: u32,
    radio: &mut Radio,
    session: &mut Option<Session>,
    scanner: &Scanner,
    stage: Stage,
    tx: &mut [u8],
) -> usize {
    if is_link(req) && !link_admits(sender, net_core_pid()) {
        return refuse_link(req, tx);
    }
    // A down radio answers through a link that is always down.
    let served = match radio {
        Radio::Up(up) => netif::serve(req, &mut up.link, tx),
        Radio::Down => netif::serve(req, &mut DeadLink, tx),
    };
    if let Some(len) = served {
        if let Radio::Up(up) = radio {
            up.link.note_netif_req();
            after_receive(up, session);
        }
        return len;
    }
    control(req, radio, session, scanner, stage, tx).unwrap_or_else(|| refuse(req, tx))
}

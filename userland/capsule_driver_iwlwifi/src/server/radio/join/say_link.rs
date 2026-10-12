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
//! One line of the link's counts when it closes, and one on why it ended.

use crate::firmware::gen3::join::inbox::Inbox;
use crate::firmware::gen3::join::run::Joined;

use super::super::say::Line;

// One line of the link's counts when it closes.
pub(super) fn say_link(j: &Joined, inbox: &Inbox, what: &[u8]) {
    let s = &j.link.stats;
    Line::new()
        .text(what)
        .text(b": tx ")
        .num(s.tx_ok)
        .text(b"/")
        .num(s.tx_drop)
        .text(b" rx ")
        .num(s.rx_eth)
        .text(b"/")
        .num(s.rx_frames)
        .text(b" refused ")
        .num(s.rx_refused)
        .text(b" full ")
        .num(s.rx_full)
        .text(b" rekeys ")
        .num(s.rekeys)
        .text(b" dropped ")
        .num(inbox.dropped)
        .text(b" stray ")
        .num(inbox.stray)
        .text(b" failed ")
        .num(inbox.tx_failed)
        .send();
}

// One line on why the link ended: the access point's reason code, or none
// when it went unheard.
pub(super) fn say_loss(left: Option<u16>) {
    let mut l = Line::new();
    match left {
        Some(r) => {
            l.text(b"link: the access point ended the association, reason ").num(u32::from(r))
        }
        None => l.text(b"link: the access point went unheard and answered no probe"),
    };
    l.send();
}

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

//! IDENTIFY: the part's maker, product name and serial from its CID, and
//! whether it is an eMMC or an SD card.

use crate::emmc::{wire_status, Opened};
use crate::protocol::{encode_identify, Request, E_NODEV, MEDIUM_EMMC, MEDIUM_SD};
use crate::server::error::reply_with_status;
use crate::server::reply::body_reply;

/// A part whose name cannot be sent as printable ASCII has none to give.
pub fn serve(o: &mut Opened, req: &Request, tx: &mut [u8]) {
    let (n, sd) = match o.slot.card_disk() {
        Ok(d) => (d.names(), d.card.sd),
        Err(e) => return reply_with_status(tx, req, wire_status(e)),
    };
    let medium = if sd { MEDIUM_SD } else { MEDIUM_EMMC };
    match encode_identify(n.model(), &n.serial, medium) {
        Some(body) => body_reply(tx, req, &body),
        None => reply_with_status(tx, req, E_NODEV),
    }
}

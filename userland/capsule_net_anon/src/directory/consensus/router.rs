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

//! The `r` and `m` lines.

use crate::directory::base64::decode;
use crate::directory::lines::args;
use crate::directory::number::{ipv4, port};

use super::entry::Entry;

/*
 * r <nickname> <identity> <publication> <IP> <ORPort> <DirPort>
 *
 * The microdesc flavour omits the descriptor digest a plain network-status
 * carries, so publication is the third field and not the fourth. Counting from
 * the wrong end reads the date as an address and discards every relay.
 *
 * Publication is not read: the live consensus sets it to 2038-01-01 on every
 * relay, so it carries nothing, and the document's own window bounds freshness.
 */
pub fn parse(rest: &[u8]) -> Option<Entry> {
    let mut field = args(rest);
    let _nickname = field.next()?;
    let identity = decode(field.next()?)?;
    if identity.len() != 20 {
        return None;
    }
    let _date = field.next()?;
    let _time = field.next()?;
    let address = ipv4(field.next()?)?;
    let or_port = port(field.next()?)?;

    let mut entry = Entry { address, or_port, ..Entry::default() };
    entry.rsa_identity.copy_from_slice(&identity);
    Some(entry)
}

/// SHA-256 of the microdescriptor, unpadded base64. `None` on a partial digest:
/// it could neither be asked for nor checked on arrival.
pub fn microdesc_digest(rest: &[u8]) -> Option<[u8; 32]> {
    let raw = decode(args(rest).next()?)?;
    if raw.len() != 32 {
        return None;
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(&raw);
    Some(out)
}

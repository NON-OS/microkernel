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

//! Reading one microdescriptor's keys and exit summary.

use crate::directory::base64::decode;
use crate::directory::lines::{arg, lines};

use super::policy::exits_web;
use super::types::Microdesc;

/*
 * The RSA `onion-key` is read past, not required. microdesc_parse.c in the fork
 * moves it from NEED_KEY_1024 to OPT_KEY_1024 and stops storing it, TAP being
 * gone. A parser written to Tor's grammar rejects any microdescriptor without
 * it; live relays still publish one, so only a parser accepting both is safe
 * across the fleet.
 */
pub fn parse(body: &[u8]) -> Option<Microdesc> {
    let mut out = Microdesc::default();
    let mut seen_ntor = false;
    for line in lines(body) {
        match line.keyword {
            b"ntor-onion-key" => {
                let raw = decode(arg(line.rest, 0)?)?;
                if raw.len() != 32 {
                    return None;
                }
                out.ntor_onion_key.copy_from_slice(&raw);
                seen_ntor = true;
            }
            b"id" => {
                if arg(line.rest, 0) == Some(b"ed25519") {
                    let raw = decode(arg(line.rest, 1)?)?;
                    if raw.len() != 32 {
                        return None;
                    }
                    out.ed25519_identity.copy_from_slice(&raw);
                }
            }
            b"p" => out.exits_web = exits_web(line.rest),
            _ => {}
        }
    }
    /*
     * ntor key for the handshake, Ed25519 identity to pin the EXTEND2. The fork
     * made the latter mandatory in every descriptor.
     */
    if !seen_ntor || out.ed25519_identity.iter().all(|b| *b == 0) {
        return None;
    }
    Some(out)
}

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

//! Fetching and anchoring the authority certificates.

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::authority::AUTHORITIES;
use crate::directory::fetch::{keys_path, upper};
use crate::directory::verify::{check, parse, AnchorError, AuthorityCert};
use crate::trace;

use super::http::fetch;

/// Certificates held for authorities whose anchor check passed, by index.
///
pub fn gather(tcp_port: u32, now: u64) -> Vec<(usize, AuthorityCert)> {
    let mut out = Vec::new();
    for (index, authority) in AUTHORITIES.iter().enumerate() {
        let mut hex = [0u8; 40];
        let width = upper(&mut hex, &authority.v3ident);
        let path = keys_path(&hex[..width]);
        let Some(body) = fetch(tcp_port, authority.address, authority.dir_port, &path) else {
            continue;
        };
        let Some(cert) = parse(&body) else { continue };
        match check(&cert, &body, &authority.v3ident, now) {
            Ok(()) => out.push((index, cert)),
            Err(AnchorError::Expired) => {
                trace::say_num(b"authority signing key expired", index as u64)
            }
            Err(_) => trace::say_num(b"authority cert rejected", index as u64),
        }
    }
    trace::say_two(b"authority certs", out.len() as u64, AUTHORITIES.len() as u64);
    out
}

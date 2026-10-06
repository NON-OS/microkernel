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

//! Addresses already resolved, for as long as they are worth reusing.

use alloc::string::String;
use alloc::vec::Vec;

use spin::Mutex;

/* An address is reused for five minutes, then asked for again. */
const KEEP_MS: i64 = 300_000;
const MAX_NAMES: usize = 64;

static NAMES: Mutex<Vec<(String, [u8; 4], i64)>> = Mutex::new(Vec::new());

/// The remembered address of `host`, if it is still fresh.
pub fn cached(host: &str) -> Option<[u8; 4]> {
    let now = nonos_libc::mk_uptime_ms();
    let names = NAMES.lock();
    let (_, ip, at) = names.iter().find(|(name, _, _)| name == host)?;
    (now.wrapping_sub(*at) < KEEP_MS).then_some(*ip)
}

pub fn remember(host: &str, ip: [u8; 4]) {
    let now = nonos_libc::mk_uptime_ms();
    let mut names = NAMES.lock();
    names.retain(|(name, _, _)| name != host);
    if names.len() >= MAX_NAMES {
        names.remove(0);
    }
    names.push((String::from(host), ip, now));
}

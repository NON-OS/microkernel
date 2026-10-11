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

//! The logs inside a transaction receipt, read as parse_logs reads a page:
//! the "logs" array alone, bounded by its own bracket, every log whole and
//! removed ones dropped. Whether the transaction succeeded is
//! parse_receipt_ok's to say.

use alloc::vec::Vec;

use super::json_scan::objects;
use super::parse_logs::{log, Log};

/* The JSON array after `key` (which ends in '['), brackets included. */
fn array<'a>(resp: &'a [u8], key: &[u8]) -> Option<&'a [u8]> {
    let at = resp.windows(key.len()).position(|w| w == key)? + key.len() - 1;
    let (mut depth, mut in_string, mut escaped) = (0u32, false, false);
    for (i, &b) in resp[at..].iter().enumerate() {
        if in_string {
            match (escaped, b) {
                (true, _) => escaped = false,
                (false, b'\\') => escaped = true,
                (false, b'"') => in_string = false,
                _ => {}
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'[' => depth += 1,
            b']' if depth == 1 => return Some(&resp[at..=at + i]),
            b']' => depth -= 1,
            _ => {}
        }
    }
    None
}

/// The logs of the receipt in `resp`; None for no receipt or any unreadable
/// log.
pub fn parse_receipt_logs(resp: &[u8]) -> Option<Vec<Log>> {
    if resp.windows(13).any(|w| w == b"\"result\":null") {
        return None;
    }
    let flag = b"\"removed\":true";
    let removed = |o: &&[u8]| o.windows(flag.len()).any(|w| w == flag);
    objects(array(resp, b"\"logs\":[")?).iter().filter(|o| !removed(o)).map(|o| log(o)).collect()
}

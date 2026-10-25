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

//! An eth_getLogs answer as logs. A log the node marks removed (a reorg) is
//! dropped. Any log that does not read whole makes the whole answer None:
//! a reader never acts on a partial page.

use alloc::vec::Vec;

use super::hex_text::{hex_bytes, quantity};
use super::json_scan::{objects, string_field};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Log {
    pub address: [u8; 20],
    pub topics: Vec<[u8; 32]>,
    pub data: Vec<u8>,
    pub block: u64,
    pub tx: [u8; 32],
}

fn topics(obj: &[u8]) -> Option<Vec<[u8; 32]>> {
    let pat = b"\"topics\":[";
    let start = obj.windows(pat.len()).position(|w| w == pat)? + pat.len();
    let len = obj[start..].iter().position(|b| *b == b']')?;
    obj[start..start + len]
        .split(|b| *b == b',')
        .filter(|t| !t.trim_ascii().is_empty())
        .map(|t| {
            hex_bytes(t.trim_ascii().strip_prefix(b"\"")?.strip_suffix(b"\"")?)?.try_into().ok()
        })
        .collect()
}

pub(crate) fn log(obj: &[u8]) -> Option<Log> {
    Some(Log {
        address: hex_bytes(string_field(obj, b"address")?)?.try_into().ok()?,
        topics: topics(obj)?,
        data: hex_bytes(string_field(obj, b"data")?)?,
        block: quantity(string_field(obj, b"blockNumber")?)?,
        tx: hex_bytes(string_field(obj, b"transactionHash")?)?.try_into().ok()?,
    })
}

/// The logs of `resp`, or None for an error answer or any unreadable log.
pub fn parse_logs(resp: &[u8]) -> Option<Vec<Log>> {
    let pat = b"\"result\":[";
    let start = resp.windows(pat.len()).position(|w| w == pat)? + pat.len() - 1;
    let flag = b"\"removed\":true";
    let removed = |o: &&[u8]| o.windows(flag.len()).any(|w| w == flag);
    objects(&resp[start..]).iter().filter(|o| !removed(o)).map(|o| log(o)).collect()
}

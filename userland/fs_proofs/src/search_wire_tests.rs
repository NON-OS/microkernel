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

// These proofs drive the production OP_SEARCH encoder rather than a copy of it,
// so a length-prefix change on either side of the wire fails here rather than
// silently corrupting the tail of a store-wide search result.

use alloc::string::String;
use alloc::vec::Vec;

use crate::files_fixture::{put, SEARCH_CONTENT, SEARCH_NAMES};
use crate::protocol::{Request, HDR_LEN, OP_SEARCH};
use crate::store::Store;
use crate::vfs_handlers::search::search;

const PID: u32 = 7;

fn search_reply(store: &mut Store, query: &str, flags: u32) -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&PID.to_le_bytes());
    payload.extend_from_slice(&flags.to_le_bytes());
    payload.extend_from_slice(&200u32.to_le_bytes());
    payload.push(query.len() as u8);
    payload.extend_from_slice(query.as_bytes());
    let req = Request { op: OP_SEARCH, flags: 0, request_id: 1, payload: &payload };
    search(store, req, PID)
}

fn decode(rx: &[u8]) -> Vec<(u32, u32, String)> {
    let mut off = HDR_LEN + 4;
    let count = u32::from_le_bytes(rx[off..off + 4].try_into().unwrap()) as usize;
    off += 4;
    let mut out = Vec::new();
    for _ in 0..count {
        let kind = u32::from_le_bytes(rx[off..off + 4].try_into().unwrap());
        let line = u32::from_le_bytes(rx[off + 4..off + 8].try_into().unwrap());
        off += 8;
        let n = rx[off] as usize;
        off += 1;
        out.push((kind, line, String::from_utf8(rx[off..off + n].to_vec()).unwrap()));
        off += n;
    }
    assert_eq!(off, rx.len(), "hit list did not consume the reply exactly");
    out
}

#[test]
fn search_opcode_follows_the_journal_pair() {
    assert_eq!(crate::protocol::OP_SEARCH, 25);
}

#[test]
fn search_reply_body_round_trips() {
    let mut s = Store::new();
    put(&mut s, "/needle.txt", b"x");
    put(&mut s, "/hay.txt", b"a\nneedle\n");

    let got = decode(&search_reply(&mut s, "needle", SEARCH_NAMES | SEARCH_CONTENT));
    let want: Vec<(u32, u32, String)> = alloc::vec![
        (0, 0, String::from("/needle.txt")),
        (1, 2, String::from("/hay.txt")),
    ];
    assert_eq!(got, want);
}

#[test]
fn a_hit_too_long_for_the_prefix_is_skipped_not_truncated() {
    let mut s = Store::new();
    let long = crate::vfs_path::normalize(&(String::from("needle") + &"a".repeat(249)));
    assert_eq!(long.len(), 256, "normalize prepends a slash, pushing 255 to 256");
    put(&mut s, &long, b"x");
    put(&mut s, "/needle.txt", b"x");

    let got = decode(&search_reply(&mut s, "needle", SEARCH_NAMES));
    let names: Vec<&str> = got.iter().map(|(_, _, p)| p.as_str()).collect();
    assert_eq!(names, ["/needle.txt"]);
}

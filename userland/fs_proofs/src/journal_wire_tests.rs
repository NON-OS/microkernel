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

// These proofs drive the production OP_JOURNAL_LIST encoder rather than a copy
// of it, so a length-prefix change on either side of the wire fails here rather
// than surfacing as a truncated Recents list at runtime.

use alloc::string::String;
use alloc::vec::Vec;

use crate::protocol::{Request, HDR_LEN, OP_JOURNAL_LIST};
use crate::store::Store;
use crate::vfs_handlers::journal::journal_list;

const PID: u32 = 7;

fn list_reply(store: &mut Store, max: u32) -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&PID.to_le_bytes());
    payload.extend_from_slice(&max.to_le_bytes());
    let req =
        Request { op: OP_JOURNAL_LIST, flags: 0, request_id: 1, payload: &payload };
    journal_list(store, req, PID)
}

fn decode(rx: &[u8]) -> Vec<(u64, String)> {
    let mut off = HDR_LEN + 4;
    let count = u32::from_le_bytes(rx[off..off + 4].try_into().unwrap()) as usize;
    off += 4;
    let mut out = Vec::new();
    for _ in 0..count {
        let atime = u64::from_le_bytes(rx[off..off + 8].try_into().unwrap());
        off += 8;
        let n = rx[off] as usize;
        off += 1;
        out.push((atime, String::from_utf8(rx[off..off + n].to_vec()).unwrap()));
        off += n;
    }
    assert_eq!(off, rx.len(), "record list did not consume the reply exactly");
    out
}

#[test]
fn journal_opcodes_follow_dirstat() {
    assert_eq!(crate::protocol::OP_JOURNAL_TOUCH, 23);
    assert_eq!(crate::protocol::OP_JOURNAL_LIST, 24);
}

#[test]
fn journal_list_reply_body_round_trips() {
    let mut s = Store::new();
    s.journal_touch("/a");
    s.journal_touch("/bb");

    let got = decode(&list_reply(&mut s, 200));
    let names: Vec<&str> = got.iter().map(|(_, p)| p.as_str()).collect();
    assert_eq!(names, ["/bb", "/a"]);
}

#[test]
fn a_name_too_long_for_the_prefix_is_skipped_not_truncated() {
    let mut s = Store::new();
    let long = crate::vfs_path::normalize(&"a".repeat(255));
    assert_eq!(long.len(), 256, "normalize prepends a slash, pushing 255 to 256");
    s.journal_touch(&long);
    s.journal_touch("/after");

    let got = decode(&list_reply(&mut s, 200));
    let names: Vec<&str> = got.iter().map(|(_, p)| p.as_str()).collect();
    assert_eq!(names, ["/after"]);
}

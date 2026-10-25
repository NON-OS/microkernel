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

//! What a load asks for: no set named gets the manifest's own; a sender's set
//! passes as sent, for the kernel to judge; an unreadable head loads nothing.

use crate::clients::vfs;
use crate::fixtures::{path, put_artifacts, EXTS, NAME};
use crate::frame::{by_name, by_name_payload as payload, frame, reply};
use crate::protocol::OP_LOAD_BY_NAME;
use crate::syscall;

const REQUIRED: u64 = 0x0000_0001_0000_1859;
const OPTIONAL: u64 = 0x0000_0000_0000_0100;

/// A head as `capsule_manifest/decode/header.rs` reads it, `cut` bytes short.
fn head(schema: u16, ns: &[u8], target: &[u8], cut: usize) -> Vec<u8> {
    let mut m = schema.to_be_bytes().to_vec();
    m.extend([0xC1; 32]);
    m.push(ns.len() as u8);
    m.extend(ns);
    m.extend([0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3]);
    m.push(target.len() as u8);
    m.extend(target);
    m.extend([0xAB; 32]);
    m.extend(REQUIRED.to_be_bytes());
    m.extend(OPTIONAL.to_be_bytes());
    m.truncate(m.len() - cut);
    m
}

fn load_with(manifest: Vec<u8>, sent: u64) -> (Vec<u8>, Vec<u64>) {
    put_artifacts(NAME, None);
    vfs::put(&path(NAME, EXTS[2]), manifest);
    syscall::reset(9);
    let r = by_name(&frame(OP_LOAD_BY_NAME, &payload(sent, &[])), 77);
    (r, syscall::take().iter().map(|l| l.req.requested_caps).collect())
}

#[test]
fn a_sender_naming_no_set_gets_the_manifest_s_own() {
    for (ns, target) in [(&b"systems.nonos.app.x"[..], &b"x86_64-nonos"[..]), (b"a", b"t")] {
        let (r, asked) = load_with(head(3, ns, target, 0), 0);
        assert_eq!(r, reply(0, &9u32.to_le_bytes()));
        assert_eq!(asked, [REQUIRED | OPTIONAL]);
    }
}

#[test]
fn a_sender_s_set_goes_to_the_kernel_as_sent() {
    for sent in [0x18, u64::MAX, REQUIRED] {
        let (_, asked) = load_with(head(3, b"ns", b"t", 0), sent);
        assert_eq!(asked, [sent], "the kernel judges it against the manifest");
    }
}

#[test]
fn a_head_this_schema_cannot_read_loads_nothing() {
    for m in [head(2, b"ns", b"t", 0), head(3, b"ns", b"t", 9), vec![], vec![0, 3]] {
        assert_eq!(load_with(m, 0), (reply(-22, &[]), vec![]));
    }
}

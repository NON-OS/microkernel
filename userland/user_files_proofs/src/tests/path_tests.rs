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

//! No path reaches above the root, names one thing two ways, or reaches a
//! link: every op refuses the same hostile paths before the tree is read.

use nonos_user_files::errno::{EACCES, EBADF, EINVAL, ENAMETOOLONG, ENOSYS};
use nonos_user_files::wire::flags::{CREATE, READ, WRITE};
use nonos_user_files::wire::{op, Header, HEADER};
use nonos_user_files::{serve, Broker};

use super::{fresh, put_file, APP};
use crate::doubles::{client, Desk, Volume};

fn hostile() -> Vec<(Vec<u8>, i32)> {
    let mut out: Vec<(Vec<u8>, i32)> = [
        &b".."[..],
        b"../etc/passwd",
        b"Documents/..",
        b"Documents/../Downloads/x",
        b"Documents/../../x",
        b"/Documents/x",
        b"Documents//x",
        b"Documents/./x",
        b"./Documents/x",
        b"Documents/x/",
        b"Documents/.",
        b"Documents/a\0b",
        b"Documents/\0",
    ]
    .iter()
    .map(|p| (p.to_vec(), EINVAL))
    .collect();
    let long_name = [b"Documents/".to_vec(), vec![b'n'; 256]].concat();
    out.push((long_name, ENAMETOOLONG));
    let long_path = [b"Documents".to_vec(), b"/abcdefg".repeat(600)].concat();
    out.push((long_path, ENAMETOOLONG));
    out
}

fn every_op(b: &mut Broker<Volume>, desk: &Desk, path: &[u8]) -> Vec<i32> {
    let mut c = client(b, desk, APP);
    let fail = |r: Result<(), i32>| r.err().unwrap_or(0);
    vec![
        fail(c.stat(path).map(drop)),
        fail(c.open(path, READ | WRITE | CREATE, 0o644).map(drop)),
        fail(c.truncate(path, 0)),
        fail(c.unlink(path)),
        fail(c.mkdir(path, 0o755).map(drop)),
        fail(c.rmdir(path)),
        fail(c.list(path, 0, 8).map(drop)),
        fail(c.setattr(path, 1, 0o777, 0).map(drop)),
        fail(c.rename(path, b"Documents/ok", 0)),
        fail(c.rename(b"Documents/ok", path, 0)),
    ]
}

#[test]
fn every_hostile_path_is_refused_by_every_op() {
    let (mut b, desk) = fresh();
    put_file(&mut b, &desk, b"Documents/ok", b"ok");
    for (path, want) in hostile() {
        let got = every_op(&mut b, &desk, &path);
        assert!(got.iter().all(|e| *e == want), "{:?}: {got:?}", String::from_utf8_lossy(&path));
    }
    assert_eq!(client(&mut b, &desk, APP).stat(b"Documents/ok").unwrap().size, 2);
}

#[test]
fn there_is_no_op_for_a_link() {
    let (mut b, desk) = fresh();
    for op in [0u16, 17, 18, 99, u16::MAX] {
        let mut msg = vec![0u8; HEADER];
        let mut head = [0u8; HEADER];
        Header { op, id: 5, body_len: 0 }.write(&mut head);
        msg.copy_from_slice(&head);
        let reply = serve(&mut b, &desk, APP, &msg);
        let status = i32::from_le_bytes(reply[HEADER..HEADER + 4].try_into().unwrap());
        assert_eq!(status, -ENOSYS, "op {op}");
    }
    let _ = op::STAT;
}

#[test]
fn a_sender_the_host_does_not_let_in_is_refused_every_op() {
    let (mut b, desk) = fresh();
    put_file(&mut b, &desk, b"Documents/ok", b"ok");
    let stranger = 99;
    let got = every_op(&mut b, &desk, b"Documents/ok");
    assert!(got.contains(&0));
    let mut c = client(&mut b, &desk, stranger);
    assert_eq!(c.stat(b"Documents/ok"), Err(EACCES));
    assert_eq!(c.open(b"Documents/ok", READ, 0).map(drop), Err(EACCES));
    assert_eq!(c.statfs().map(drop), Err(EACCES));
}

#[test]
fn a_handle_is_its_owners_alone() {
    let (mut b, desk) = fresh();
    let desk2 = Desk::new(&[APP, 8]);
    put_file(&mut b, &desk, b"Documents/ok", b"ok");
    let (h, _) = client(&mut b, &desk2, APP).open(b"Documents/ok", READ, 0).unwrap();
    let mut other = client(&mut b, &desk2, 8);
    let mut buf = [0u8; 2];
    assert_eq!(other.read(h, 0, &mut buf), Err(EBADF));
    assert_eq!(other.write(h, 0, b"x").map(drop), Err(EBADF));
    assert_eq!(other.close(h), Err(EBADF));
    assert_eq!(other.fstat(h).map(drop), Err(EBADF));
    assert_eq!(client(&mut b, &desk2, APP).read(h, 0, &mut buf), Ok(2));
}

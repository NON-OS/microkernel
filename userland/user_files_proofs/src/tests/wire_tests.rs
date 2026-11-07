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

//! Every op through the client and the wire, and the errno each refusal
//! carries.

use nonos_user_files::errno::*;
use nonos_user_files::wire::flags::*;
use nonos_user_files::Kind;

use super::{fresh, APP};
use crate::doubles::client;

#[test]
fn a_first_start_has_the_persons_folders_and_nothing_else() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    let root = c.list(b"", 0, 64).unwrap();
    let names: Vec<&[u8]> = root.entries.iter().map(|e| e.2.as_slice()).collect();
    assert_eq!(names, [&b"Documents"[..], b"Downloads", b"Music", b"Pictures", b"Videos"]);
    assert!(root.end);
    assert_eq!(c.stat(b"Documents").unwrap().kind, Kind::Dir);
    assert!(c.list(b"Documents", 0, 64).unwrap().entries.is_empty());
}

#[test]
fn a_file_is_written_read_sized_and_listed() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    let (h, a) = c.open(b"Documents/letter.odt", READ | WRITE | CREATE, 0o644).unwrap();
    assert_eq!((a.kind, a.size, a.mode, a.nlink), (Kind::File, 0, 0o644, 1));
    assert_eq!(c.write(h, 0, b"hello world").unwrap(), (11, 11));
    let mut buf = [0u8; 64];
    assert_eq!(c.read(h, 6, &mut buf).unwrap(), 5);
    assert_eq!(&buf[..5], b"world");
    assert_eq!(c.read(h, 11, &mut buf).unwrap(), 0);
    c.ftruncate(h, 5).unwrap();
    assert_eq!(c.fstat(h).unwrap().size, 5);
    c.fsync(h).unwrap();
    c.close(h).unwrap();
    assert_eq!(c.close(h), Err(EBADF));
    let listed = c.list(b"Documents", 0, 64).unwrap();
    assert_eq!(listed.entries.len(), 1);
    assert_eq!(listed.entries[0].2, b"letter.odt");
    assert_eq!(c.stat(b"Documents/letter.odt").unwrap().size, 5);
}

#[test]
fn names_are_made_moved_and_removed() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    c.mkdir(b"Documents/work", 0o755).unwrap();
    assert_eq!(c.mkdir(b"Documents/work", 0o755), Err(EEXIST));
    let (h, _) = c.open(b"Documents/work/a.txt", WRITE | CREATE, 0o600).unwrap();
    c.write(h, 0, b"a").unwrap();
    c.close(h).unwrap();
    assert_eq!(c.rmdir(b"Documents/work"), Err(ENOTEMPTY));
    c.rename(b"Documents/work/a.txt", b"Downloads/a.txt", 0).unwrap();
    assert_eq!(c.stat(b"Documents/work/a.txt"), Err(ENOENT));
    assert_eq!(c.stat(b"Downloads/a.txt").unwrap().size, 1);
    c.rmdir(b"Documents/work").unwrap();
    assert_eq!(c.unlink(b"Downloads"), Err(EISDIR));
    c.unlink(b"Downloads/a.txt").unwrap();
    assert_eq!(c.unlink(b"Downloads/a.txt"), Err(ENOENT));
    c.truncate(b"Documents", 0).unwrap_err();
    let t = c.statfs().unwrap();
    assert_eq!((t.used, t.nodes, t.chunk), (0, 6, 1 << 20));
}

#[test]
fn open_flags_follow_linux() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    let p = b"Documents/f";
    assert_eq!(c.open(p, READ, 0), Err(ENOENT));
    let (h, _) = c.open(p, WRITE | CREATE | EXCL, 0o644).unwrap();
    assert_eq!(c.open(p, WRITE | CREATE | EXCL, 0o644).map(|x| x.0), Err(EEXIST));
    c.write(h, 0, b"0123456789").unwrap();
    let mut buf = [0u8; 4];
    assert_eq!(c.read(h, 0, &mut buf), Err(EBADF));
    let (a, _) = c.open(p, WRITE | APPEND, 0).unwrap();
    assert_eq!(c.write(a, 0, b"AB").unwrap(), (2, 12));
    let (t, attr) = c.open(p, WRITE | TRUNC, 0).unwrap();
    assert_eq!(attr.size, 0);
    let (r, _) = c.open(p, READ, 0).unwrap();
    assert_eq!(c.write(r, 0, b"x"), Err(EBADF));
    assert_eq!(c.ftruncate(r, 0), Err(EINVAL));
    assert_eq!(c.open(p, READ | DIRECTORY, 0).map(|x| x.0), Err(ENOTDIR));
    assert_eq!(c.open(b"Documents", WRITE, 0).map(|x| x.0), Err(EISDIR));
    let (d, _) = c.open(b"Documents", READ | DIRECTORY, 0).unwrap();
    assert_eq!(c.read(d, 0, &mut buf), Err(EISDIR));
    assert_eq!(c.open(p, 0, 0).map(|x| x.0), Err(EINVAL));
    assert_eq!(c.open(p, READ | 1 << 9, 0).map(|x| x.0), Err(EINVAL));
    for h in [h, a, t, r, d] {
        c.close(h).unwrap();
    }
}

#[test]
fn the_root_holds_only_the_two_folders() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    assert_eq!(c.open(b"stray", WRITE | CREATE, 0o644).map(|x| x.0), Err(EACCES));
    assert_eq!(c.mkdir(b"stray", 0o755).map(drop), Err(EACCES));
    assert_eq!(c.mkdir(b"Documents", 0o755).map(drop), Err(EEXIST));
    assert_eq!(c.rmdir(b"Documents"), Err(EBUSY));
    assert_eq!(c.rmdir(b""), Err(EBUSY));
    assert_eq!(c.rename(b"Documents", b"Documents/x", 0), Err(EBUSY));
    let (h, _) = c.open(b"Documents/x", WRITE | CREATE, 0o644).unwrap();
    c.close(h).unwrap();
    assert_eq!(c.rename(b"Documents/x", b"x", 0), Err(EACCES));
    assert_eq!(c.rename(b"Documents/x", b"Downloads", 0), Err(EBUSY));
}

#[test]
fn rename_flags_follow_renameat2() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    for (name, body) in [(&b"Documents/a"[..], &b"A"[..]), (b"Documents/b", b"BB")] {
        let (h, _) = c.open(name, WRITE | CREATE, 0o644).unwrap();
        c.write(h, 0, body).unwrap();
        c.close(h).unwrap();
    }
    assert_eq!(c.rename(b"Documents/a", b"Documents/b", RENAME_NOREPLACE), Err(EEXIST));
    c.rename(b"Documents/a", b"Documents/b", RENAME_EXCHANGE).unwrap();
    assert_eq!(c.stat(b"Documents/a").unwrap().size, 2);
    assert_eq!(c.stat(b"Documents/b").unwrap().size, 1);
    assert_eq!(c.rename(b"Documents/a", b"Documents/z", RENAME_EXCHANGE), Err(ENOENT));
    assert_eq!(c.rename(b"Documents/a", b"Documents/b", 3), Err(EINVAL));
    c.mkdir(b"Documents/d", 0o755).unwrap();
    c.mkdir(b"Documents/d/e", 0o755).unwrap();
    assert_eq!(c.rename(b"Documents/d", b"Documents/d/e/f", 0), Err(EINVAL));
    assert_eq!(c.rename(b"Documents/a", b"Documents/d", 0), Err(EISDIR));
    assert_eq!(c.rename(b"Documents/d", b"Documents/a", 0), Err(ENOTDIR));
    c.rename(b"Documents/a", b"Documents/a", 0).unwrap();
}

#[test]
fn setattr_sets_mode_and_mtime() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    let (h, _) = c.open(b"Documents/m", WRITE | CREATE, 0o644).unwrap();
    c.close(h).unwrap();
    let a = c.setattr(b"Documents/m", SET_MODE | SET_MTIME, 0o100_600, 42).unwrap();
    assert_eq!((a.mode, a.mtime), (0o600, 42));
    let a = c.setattr(b"Documents/m", SET_MODE, 0o640, 7).unwrap();
    assert_eq!((a.mode, a.mtime), (0o640, 42));
}

#[test]
fn a_listing_comes_in_pieces_by_cookie() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    for i in 0..1200 {
        let (h, _) =
            c.open(format!("Documents/f{i:04}").as_bytes(), WRITE | CREATE, 0o644).unwrap();
        c.close(h).unwrap();
    }
    let (mut cookie, mut names) = (0, Vec::new());
    loop {
        let l = c.list(b"Documents", cookie, 10_000).unwrap();
        assert!(l.entries.len() <= 512);
        names.extend(l.entries.into_iter().map(|e| e.2));
        cookie = l.next;
        if l.end {
            break;
        }
    }
    assert_eq!(names.len(), 1200);
    assert!(names.windows(2).all(|w| w[0] < w[1]));
}

#[test]
fn no_folder_is_removed_renamed_or_taken_by_a_file() {
    let (mut b, desk) = fresh();
    let mut c = client(&mut b, &desk, APP);
    for f in [&b"Documents"[..], b"Downloads", b"Music", b"Pictures", b"Videos"] {
        assert_eq!(c.rmdir(f), Err(EBUSY), "{f:?}");
        assert_eq!(c.rename(f, b"Documents/x", 0), Err(EBUSY));
        assert_eq!(c.mkdir(f, 0o755).map(drop), Err(EEXIST));
        assert_eq!(c.open(f, WRITE, 0).map(drop), Err(EISDIR));
    }
    let (h, _) = c.open(b"Pictures/cat.png", WRITE | CREATE, 0o644).unwrap();
    c.write(h, 0, b"png").unwrap();
    c.close(h).unwrap();
    assert_eq!(c.rename(b"Pictures/cat.png", b"Music", 0), Err(EBUSY));
    c.rename(b"Pictures/cat.png", b"Videos/cat.png", 0).unwrap();
    assert_eq!(c.stat(b"Videos/cat.png").unwrap().size, 3);
}

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

//! Two programs on the same files at once.

use nonos_user_files::errno::{EEXIST, EMFILE};
use nonos_user_files::limits::HANDLES_PER_CLIENT;
use nonos_user_files::wire::flags::{CREATE, EXCL, READ, TRUNC, WRITE};

use super::live::no_strays;
use super::{fresh, get_file, pattern, put_file, settle, APP};
use crate::doubles::{client, Desk};

const OTHER: u32 = 8;

#[test]
fn one_programs_write_is_the_others_read() {
    let (mut b, _) = fresh();
    let desk = Desk::new(&[APP, OTHER]);
    let (w, _) =
        client(&mut b, &desk, APP).open(b"Documents/shared", WRITE | CREATE, 0o644).unwrap();
    let (r, _) = client(&mut b, &desk, OTHER).open(b"Documents/shared", READ, 0).unwrap();
    client(&mut b, &desk, APP).write(w, 0, b"first").unwrap();
    let mut buf = [0u8; 16];
    assert_eq!(client(&mut b, &desk, OTHER).read(r, 0, &mut buf).unwrap(), 5);
    client(&mut b, &desk, APP).write(w, 1_048_570, b"across").unwrap();
    assert_eq!(client(&mut b, &desk, OTHER).read(r, 1_048_570, &mut buf).unwrap(), 6);
    assert_eq!(&buf[..6], b"across");
    assert_eq!(client(&mut b, &desk, OTHER).fstat(r).unwrap().size, 1_048_576);
}

#[test]
fn of_two_exclusive_creates_one_wins() {
    let (mut b, _) = fresh();
    let desk = Desk::new(&[APP, OTHER]);
    let a = client(&mut b, &desk, APP).open(b"Documents/lock", WRITE | CREATE | EXCL, 0o600);
    let o = client(&mut b, &desk, OTHER).open(b"Documents/lock", WRITE | CREATE | EXCL, 0o600);
    assert!(a.is_ok());
    assert_eq!(o.map(drop), Err(EEXIST));
}

#[test]
fn a_reader_keeps_the_old_document_while_the_other_saves_over_it() {
    let (mut b, _) = fresh();
    let desk = Desk::new(&[APP, OTHER]);
    put_file(&mut b, &desk, b"Documents/doc", &pattern(1, 3_000_000));
    let (r, _) = client(&mut b, &desk, OTHER).open(b"Documents/doc", READ, 0).unwrap();
    put_file(&mut b, &desk, b"Documents/doc.new", &pattern(2, 1_000_000));
    client(&mut b, &desk, APP).rename(b"Documents/doc.new", b"Documents/doc", 0).unwrap();
    let mut buf = vec![0u8; 4096];
    let n = client(&mut b, &desk, OTHER).read(r, 2_500_000, &mut buf).unwrap();
    assert_eq!(&buf[..n], &pattern(1, 3_000_000)[2_500_000..2_504_096]);
    assert_eq!(get_file(&mut b, &desk, b"Documents/doc").unwrap(), pattern(2, 1_000_000));
    client(&mut b, &desk, OTHER).close(r).unwrap();
    settle(&mut b, &desk);
    no_strays(b.objects());
}

#[test]
fn a_program_that_ends_has_its_files_closed_and_its_writes_kept() {
    let (mut b, _) = fresh();
    let mut desk = Desk::new(&[APP, OTHER]);
    let (h, _) =
        client(&mut b, &desk, OTHER).open(b"Documents/w", WRITE | CREATE | TRUNC, 0o644).unwrap();
    client(&mut b, &desk, OTHER).write(h, 0, &pattern(3, 2_100_000)[..200_000]).unwrap();
    desk.running.remove(&OTHER);
    b.reap(&desk);
    settle(&mut b, &desk);
    let mut next = nonos_user_files::Broker::new(
        b.objects().reboot(),
        nonos_user_files::limits::DEFAULT_QUOTA,
    );
    assert_eq!(
        get_file(&mut next, &desk, b"Documents/w").unwrap(),
        pattern(3, 2_100_000)[..200_000]
    );
    no_strays(next.objects());
}

#[test]
fn one_program_holding_its_share_of_handles_leaves_the_other_room() {
    let (mut b, _) = fresh();
    let desk = Desk::new(&[APP, OTHER]);
    put_file(&mut b, &desk, b"Documents/f", b"f");
    for _ in 0..HANDLES_PER_CLIENT {
        client(&mut b, &desk, APP).open(b"Documents/f", READ, 0).unwrap();
    }
    assert_eq!(client(&mut b, &desk, APP).open(b"Documents/f", READ, 0).map(drop), Err(EMFILE));
    assert!(client(&mut b, &desk, OTHER).open(b"Documents/f", READ, 0).is_ok());
}

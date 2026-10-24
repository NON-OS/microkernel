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

//! Files are opened once, never through a link, never waited on: a FIFO, a
//! link or a file past its bound is refused at once; and a verdict is
//! written into a file created for it, never through a name planted there.

use crate::hash::{sha256_file, MAX_FILE};
use crate::release::read::{open_regular, read_regular};
use crate::write::write_whole;
use std::fs;
use std::os::unix::fs::symlink;
use std::time::{Duration, Instant};

#[test]
fn a_fifo_a_link_or_an_oversized_file_is_refused_at_once() {
    let dir = tempfile::tempdir().unwrap();
    let (fifo, link, big) =
        (dir.path().join("fifo"), dir.path().join("link"), dir.path().join("big"));
    let c = std::ffi::CString::new(fifo.to_str().unwrap()).unwrap();
    assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
    fs::write(dir.path().join("real"), b"x").unwrap();
    symlink(dir.path().join("real"), &link).unwrap();
    fs::File::create(&big).unwrap().set_len(MAX_FILE + 1).unwrap();
    let started = Instant::now();
    assert!(open_regular(&fifo).is_err() && sha256_file(&fifo).is_err());
    assert!(open_regular(&link).is_err_and(|e| e.contains("link")));
    assert!(sha256_file(&big).is_err() && read_regular(&big, 16).is_err());
    assert!(started.elapsed() < Duration::from_secs(5), "something was waited on");
}

#[test]
fn a_link_planted_beside_the_verdict_is_not_written_through() {
    let dir = tempfile::tempdir().unwrap();
    let (out, victim) = (dir.path().join("verdict.json"), dir.path().join("victim"));
    fs::write(&victim, b"the operator's file").unwrap();
    symlink(&victim, dir.path().join("verdict.json.partial")).unwrap();
    symlink(&victim, dir.path().join("verdict.json.sig.partial")).unwrap();
    write_whole(&out, b"the verdict").unwrap();
    assert_eq!(fs::read(&victim).unwrap(), b"the operator's file");
    assert_eq!(fs::read(&out).unwrap(), b"the verdict");
    assert!(!fs::symlink_metadata(&out).unwrap().file_type().is_symlink());
}

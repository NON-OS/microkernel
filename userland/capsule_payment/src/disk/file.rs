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

use alloc::vec::Vec;

use nonos_payment_proto::{EAGAIN, ENOPERSIST};
use nonos_policy_client::{get_bool, lookup};
use nonos_policy_proto::Field;

use super::chunked::{read_all, write_all};
use super::ops;
use super::paths::DIR;
use super::vfs::{ENOENT, O_CREATE, O_TRUNC};

pub enum Loaded {
    Bytes(Vec<u8>),
    Absent,
    /// The store did not answer, or the file is not `len` bytes: it says
    /// nothing a caller may act on, and is asked again.
    Silent,
}

pub fn load(path: &[u8], len: usize) -> Loaded {
    let fd = match ops::open(path, 0) {
        Ok(fd) => fd,
        Err(Some(ENOENT)) => return Loaded::Absent,
        Err(_) => return Loaded::Silent,
    };
    let got = read_all(fd, len);
    ops::close(fd);
    got.map_or(Loaded::Silent, Loaded::Bytes)
}

/// Whether this boot keeps data past power off. A policy service that does
/// not answer is taken as one that keeps nothing, as the store does.
pub fn kept() -> bool {
    lookup().and_then(|port| get_bool(port, Field::Persistent)) == Some(true)
}

/// Write `data` and persist it, or ENOPERSIST on a boot that keeps nothing: the
/// meter changes no state it could not find again. A record restored at boot
/// belongs to no process, and the store persists only a file its caller
/// made, so the name is made again before every write; the record on disk is
/// replaced only by the persist, whole or not at all.
pub fn save(path: &[u8], data: &[u8]) -> Result<(), i32> {
    if !kept() {
        return Err(ENOPERSIST);
    }
    let _ = ops::mkdir(DIR);
    let _ = ops::unlink(path);
    let fd = ops::open(path, O_CREATE | O_TRUNC).map_err(code)?;
    let wrote = write_all(fd, data);
    ops::close(fd);
    wrote.map_err(code)?;
    ops::persist(path).map_err(code)
}

fn code(e: Option<i32>) -> i32 {
    e.unwrap_or(EAGAIN)
}

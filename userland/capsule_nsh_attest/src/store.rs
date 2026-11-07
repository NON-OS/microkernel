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

//! The machine's store, through vfs: the sealed host keys nsh.attest keeps,
//! the enrolment request it writes for the operator, and the records the
//! operator puts back. Every path is under /nonos/nsh. A file is written
//! afresh (unlinked, written, persisted), since vfs lets only a file's
//! owner persist it and a file kept from an earlier boot belongs to
//! nobody. Every call waits at most CALL_MS, so a vfs that does not answer
//! costs one connection a refusal, not the capsule.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

/// The directory every nsh record is in.
pub const DIR: &[u8] = b"/nonos/nsh";
/// The longest one vfs round trip may take.
const CALL_MS: u64 = 2_000;

/// What reading one file found.
#[derive(Debug, PartialEq, Eq)]
pub enum Found {
    /// The file's bytes, at most the cap asked for.
    Bytes(Vec<u8>),
    /// No such file.
    Absent,
    /// The file is longer than the cap asked for.
    Over,
    /// vfs did not answer, or refused.
    Fault,
}

/// Bound every vfs call this process makes.
pub fn bound_calls() {
    vfs::set_deadline_ms(CALL_MS);
}

/// What vfs says of its store on this boot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disk {
    /// vfs has not finished loading its store, or did not answer: a file
    /// not seen yet may still be on the disk.
    Unsettled,
    /// vfs settled with no NONOS disk: nothing written is kept.
    None,
    /// vfs settled from a NONOS disk.
    Read,
}

/// The store's state now.
pub fn disk() -> Disk {
    match vfs::store_settled() {
        Ok(true) if vfs::store_has_disk() => Disk::Read,
        Ok(true) => Disk::None,
        _ => Disk::Unsettled,
    }
}

/// The file at `path`, if it is at most `cap` bytes.
pub fn read(path: &[u8], cap: usize) -> Found {
    let pid = mk_getpid();
    match vfs::lookup(pid, path) {
        Ok(Some(_)) => {}
        Ok(None) => return Found::Absent,
        Err(_) => return Found::Fault,
    }
    let Some(ask) = cap.checked_add(1).and_then(|n| u32::try_from(n).ok()) else {
        return Found::Fault;
    };
    match vfs::read_file(pid, path, ask) {
        Ok(bytes) if bytes.len() > cap => Found::Over,
        Ok(bytes) => Found::Bytes(bytes),
        Err(_) => Found::Fault,
    }
}

/// Write `bytes` to `path` afresh and keep it. `false` when it could not
/// be written, or written but not kept (a boot that keeps nothing).
pub fn put(path: &[u8], bytes: &[u8]) -> bool {
    let pid = mk_getpid();
    let _ = vfs::mkdir(pid, b"/nonos");
    let _ = vfs::mkdir(pid, DIR);
    let _ = vfs::unlink(pid, path);
    vfs::write_file(pid, path, bytes).is_ok() && vfs::persist(pid, path).is_ok()
}

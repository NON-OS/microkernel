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

//! When a writer's pages go back to its file: before any read of the bytes
//! it shows, before the family's copy of its file goes to the store (at
//! close, fsync, sync and exit), and at msync. A file reads, and is kept,
//! with what its maps wrote, as the page cache would have it on Linux.

use alloc::vec::Vec;

use super::back::back;
use super::spans::going;
use super::table::picked;

/// Before `[at, at + len)` of `path` is read.
pub fn before_read(path: &[u8], at: u64, len: u64) {
    for e in picked(|e| e.writer && e.shows(path)) {
        back(&e, (at, at.saturating_add(len)));
    }
}

/// Before the family's copy of `path` goes to the store.
pub fn before_flush(path: &[u8]) {
    for e in picked(|e| e.writer && e.shows(path)) {
        back(&e, e.file());
    }
}

/// Before every copy goes to the store: sync(2), and a process's exit.
pub fn before_flush_all() {
    for e in picked(|e| e.writer && !e.orphan) {
        back(&e, e.file());
    }
}

/// msync of `[at, at + span)` of `pid`: what writers changed there goes
/// back, then each file shown there is synced as fsync syncs it.
pub fn synced(pid: u32, at: u64, span: u64) -> Result<(), i64> {
    let mut paths: Vec<Vec<u8>> = Vec::new();
    for e in picked(|e| e.pid == pid && !e.orphan) {
        let Some(part) = going(&e, at, span) else {
            continue;
        };
        back(&e, part);
        if !paths.contains(&e.path) {
            paths.push(e.path);
        }
    }
    paths.iter().try_for_each(|path| super::super::super::fsync::sync_path(path))
}

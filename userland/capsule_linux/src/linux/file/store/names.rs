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

//! A stored file written, and what the store says of a name.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{self, VfsStream};
use nonos_libc::mk_getpid;

use super::super::root::Key;
use super::bytes::Fail;
use super::packed::{packed, packed_stat};
use super::stream::Stream;

pub fn write(at: &Key, data: &[u8]) -> Result<(), Fail> {
    at.writable()?;
    vfs::write_file(mk_getpid(), at.as_bytes(), data)
}

/*
 * The store keeps files and no directories: /linux/bin exists only as the
 * prefix of what is in it. A key that is no file but has keys below it is
 * answered as a directory, or `ls /bin` finds nothing to list.
 */
pub fn stat(at: &Key) -> Result<(u64, bool), Fail> {
    vfs::stat(mk_getpid(), at.as_bytes())
        .or_else(|e| packed_stat(at).or_else(|| implicit_dir(at).map(|_| (0, true))).ok_or(e))
}

/// A packed file is read-only and as old as the epoch: its pack keeps no time.
pub fn stat_full(at: &Key) -> Result<(u64, bool, u64, bool), Fail> {
    vfs::stat_full(mk_getpid(), at.as_bytes()).or_else(|e| {
        let found = packed_stat(at).or_else(|| implicit_dir(at).map(|_| (0, true)));
        found.map(|(size, dir)| (size, dir, 0, false)).ok_or(e)
    })
}

fn implicit_dir(at: &Key) -> Option<()> {
    let below = list(at).ok()?;
    let prefix = at.as_bytes();
    below.iter().any(|k| k.as_bytes().get(prefix.len()) == Some(&b'/')).then_some(())
}

/// The store's keys below `at`, and the packs' in the shared tree, as keys.
pub fn list(at: &Key) -> Result<Vec<String>, Fail> {
    let mut keys = vfs::list_paths(mk_getpid(), at.as_bytes());
    if let (Some(v), Ok(keys)) = (packed(at), keys.as_mut()) {
        let root = &at.0[..at.2];
        let packed = super::pack::list(v).into_iter().map(|p| [root, &p].concat());
        keys.extend(packed.filter_map(|k| String::from_utf8(k).ok()));
    }
    keys
}

pub fn open(at: &Key) -> Result<Stream, Fail> {
    VfsStream::open(mk_getpid(), at.as_bytes()).map(Stream::Store).or_else(|e| {
        let read = packed(at).and_then(|v| super::pack::read(v, u32::MAX));
        read.unwrap_or(Err(e)).map(Stream::Packed)
    })
}

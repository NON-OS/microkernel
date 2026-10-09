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

/*
 * The one streamed import in flight: whose it is, what it must become, and
 * how far it has come. It lives in memory between the calls that feed it;
 * its last saved mark on the volume is what outlives a reboot.
 */

use spin::Mutex;

use super::hash::PinHash;
use crate::fs::blockfs::FileStream;

/*
 * The longest stream name: its slash, then a file name that leaves room in
 * a directory entry's 56 bytes for its mark's `.partial`. A longer one
 * could never be marked, so it is refused before a byte is sealed.
 */
pub(super) const NAME_MAX: usize = 1 + 56 - b".partial".len();
/* Bytes sealed between two saved marks. A cut loses at most this much. */
pub(super) const MARK_EVERY: u64 = 64 << 20;

pub(super) struct Live {
    pub(super) pid: u32,
    pub(super) name: [u8; NAME_MAX],
    pub(super) name_len: usize,
    pub(super) want: [u8; 32],
    pub(super) bytes: u64,
    pub(super) stream: FileStream,
    pub(super) hash: PinHash,
    /* Where the last mark on the volume stands. */
    pub(super) marked: u64,
}

impl Live {
    pub(super) fn new(
        pid: u32,
        name: &[u8],
        want: &[u8; 32],
        bytes: u64,
        stream: FileStream,
        hash: PinHash,
    ) -> Self {
        let mut n = [0u8; NAME_MAX];
        n[..name.len()].copy_from_slice(name);
        let (name_len, want, marked) = (name.len(), *want, stream.size());
        Live { pid, name: n, name_len, want, bytes, stream, hash, marked }
    }

    pub(super) fn name(&self) -> &[u8] {
        &self.name[..self.name_len]
    }

    /* Whether this is the stream for `name`, pinned to `want` and `bytes`. */
    pub(super) fn is(&self, name: &[u8], want: &[u8; 32], bytes: u64) -> bool {
        self.name() == name && &self.want == want && self.bytes == bytes
    }
}

pub(super) static LIVE: Mutex<Option<Live>> = Mutex::new(None);

/*
 * `f`, run while no stream is coming to `name`, with the stream lock held so
 * none can begin under it; Importing when one is. The stream lock is always
 * taken before the volume's, as a feed takes them.
 */
pub(in super::super) fn without_stream<R>(
    name: &[u8],
    f: impl FnOnce() -> Result<R, super::super::error::VolumeError>,
) -> Result<R, super::super::error::VolumeError> {
    let live = LIVE.lock();
    if live.as_ref().is_some_and(|l| &l.name[..l.name_len] == name) {
        return Err(super::super::error::VolumeError::Importing);
    }
    let done = f();
    drop(live);
    done
}

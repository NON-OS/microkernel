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
 * Beginning a fed import, or taking one up again: from the stream in memory
 * when its owner has gone, from its mark on the volume after a reboot, or
 * from nothing. A probe says where one would start and holds nothing. A
 * record or mark name is never streamed: the kernel writes every one. */

use super::super::import_guard::is_kept;
use super::super::import_record::recorded;
use super::super::open_machine::open_machine_volume;
use super::error::StreamError;
use super::live::{Live, LIVE, NAME_MAX};
use super::owner::alive;
use super::pause::pause;
use super::resume::resume;

/* Where a stream stands when it begins. */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Begun {
    /* Imported and verified before, this many bytes; nothing to feed. */
    Done(u64),
    /* Feed it on from this byte. */
    From(u64),
}

/* Begin feeding `name`, `bytes` long with SHA-256 `want`, for `pid`. */
pub fn stream_begin(
    pid: u32,
    name: &[u8],
    want: &[u8; 32],
    bytes: u64,
    probe: bool,
) -> Result<Begun, StreamError> {
    if name.is_empty() || name.len() > NAME_MAX || is_kept(name) {
        return Err(StreamError::NotBegun);
    }
    open_machine_volume()?;
    if let Some(size) = recorded(name, want)? {
        return Ok(Begun::Done(size));
    }
    let mut live = LIVE.lock();
    if let Some(l) = live.as_mut() {
        let owned = l.pid == pid || !alive(l.pid);
        if l.is(name, want, bytes) && (owned || probe) {
            l.pid = if probe { l.pid } else { pid };
            return Ok(Begun::From(l.stream.size()));
        }
        if !probe {
            pause(if owned { live.take() } else { return Err(StreamError::Busy) })?;
        }
    }
    let (hash, stream) = resume(name, want, bytes)?;
    let at = stream.size();
    if !probe {
        *live = Some(Live::new(pid, name, want, bytes, stream, hash));
    }
    Ok(Begun::From(at))
}

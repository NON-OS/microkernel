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
 * Saving a fed import's mark, and putting a stream down with its mark
 * saved, so the next begin, in this boot or after a reboot, goes on from it.
 */

use super::super::error::VolumeError;
use super::super::state::VOLUME;
use super::error::StreamError;
use super::live::Live;
use super::mark::save;
use super::mark_codec::encode;

/*
 * Save `l`'s mark on the volume. Writing it commits the volume, and with it
 * every block the stream sealed before it.
 */
pub(super) fn mark(l: &mut Live) -> Result<(), VolumeError> {
    let body = encode(&l.want, l.bytes, &l.hash, &l.stream);
    let mut guard = VOLUME.write();
    let s = guard.as_mut().ok_or(VolumeError::NotMounted)?;
    save(&s.key, &mut s.mount, l.name(), &body)?;
    l.marked = l.stream.size();
    Ok(())
}

/*
 * Put `live` down, its mark saved when it has come on since the last one.
 * Where it stood.
 */
pub(super) fn pause(live: Option<Live>) -> Result<u64, StreamError> {
    let mut l = live.ok_or(StreamError::NotBegun)?;
    if l.stream.size() > l.marked {
        mark(&mut l)?;
    }
    Ok(l.stream.size())
}

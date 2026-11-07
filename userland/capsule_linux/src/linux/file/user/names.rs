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

//! The calls that change a name in the person's folders.

use nonos_user_files::wire::flags::{SET_MODE, SET_MTIME};

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::{link, place};

fn answer(r: Result<(), i32>) -> u64 {
    r.map_or_else(|e| errno::fail(e as i64), |()| errno::ok(0))
}

/// mkdir and mkdirat.
pub fn mkdir(guest: &Guest, at: &[u8], mode: u64) -> Option<u64> {
    let path = place::served(guest, at)?;
    Some(answer(link::with(|c| c.mkdir(path, (mode & 0o7777) as u32).map(drop))))
}

/// unlink, unlinkat, and rmdir as unlinkat's AT_REMOVEDIR.
pub fn unlink(guest: &Guest, at: &[u8], dir: bool) -> Option<u64> {
    let path = place::served(guest, at)?;
    Some(answer(link::with(|c| if dir { c.rmdir(path) } else { c.unlink(path) })))
}

/// rename and renameat2 with RENAME_NOREPLACE or RENAME_EXCHANGE. A move
/// between the folders and anywhere else is EXDEV, as between two mounts:
/// mv and file managers copy then.
pub fn rename(guest: &Guest, from: &[u8], to: &[u8], flags: u64) -> Option<u64> {
    match (place::served(guest, from), place::served(guest, to)) {
        (None, None) => None,
        (Some(f), Some(t)) => match u32::try_from(flags) {
            Ok(flags) if flags & !3 == 0 => Some(answer(link::with(|c| c.rename(f, t, flags)))),
            _ => Some(errno::fail(errno::EINVAL)),
        },
        _ => Some(errno::fail(errno::EXDEV)),
    }
}

/// chmod, fchmod and fchmodat.
pub fn chmod(guest: &Guest, full: &[u8], mode: u64) -> Option<u64> {
    let path = place::served(guest, full)?;
    let mode = (mode & 0o7777) as u32;
    Some(answer(link::with(|c| c.setattr(path, SET_MODE, mode, 0).map(drop))))
}

/// utimensat and utimes: the broker keeps the modification time; an access
/// time alone changes nothing it keeps.
pub fn stamp(guest: &Guest, full: &[u8], mtime_ms: Option<u64>) -> Option<u64> {
    let path = place::served(guest, full)?;
    let Some(ms) = mtime_ms else {
        return Some(errno::ok(0));
    };
    let ns = ms.saturating_mul(1_000_000);
    Some(answer(link::with(|c| c.setattr(path, SET_MTIME, 0, ns).map(drop))))
}

/// A hard or symbolic link may not be made in the folders: the broker keeps
/// neither, so nothing in them can point out of them.
pub fn no_link(guest: &Guest, at: &[u8]) -> Option<i64> {
    place::served(guest, at).map(|_| errno::EPERM)
}

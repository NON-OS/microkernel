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

//! Where the terminal's `linux` command finds a program. A name with a slash
//! is a path in the Linux tree; a bare name is looked for along the PATH a
//! guest starts with, the first directory that holds it winning, as a
//! shell's search goes.

use alloc::vec::Vec;

use crate::linux::file::{key, store_stat, visible};
use crate::linux::guest::Links;

/// The name as found, and the file it leads to through the image's links.
/// A bare name no PATH directory holds is given as /bin's, which the
/// caller then fails to read and says so.
pub(super) fn find(program: &[u8], links: Option<&Links>) -> (Vec<u8>, Vec<u8>) {
    let resolve = |named: Vec<u8>| {
        let path = links.map_or_else(|| named.clone(), |l| l.follow(named.clone(), true));
        (named, path)
    };
    if program.contains(&b'/') {
        return resolve(visible(b"/", program));
    }
    for dir in path_dirs() {
        let (named, path) = resolve(visible(&dir, program));
        if matches!(store_stat(&key(&path)), Ok((_, false))) {
            return (named, path);
        }
    }
    resolve(visible(b"/bin", program))
}

/// The directories of the PATH in the environment a guest starts with.
fn path_dirs() -> Vec<Vec<u8>> {
    let env = crate::linux::env::default();
    let path = env.iter().find_map(|v| v.strip_prefix(b"PATH=")).unwrap_or(b"/bin");
    path.split(|b| *b == b':').filter(|d| !d.is_empty()).map(<[u8]>::to_vec).collect()
}

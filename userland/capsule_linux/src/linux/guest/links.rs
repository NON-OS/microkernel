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

//! Symbolic links, as a table the image carries.
//!
//! The store has files and nothing else, and a distribution leans on links:
//! busybox finds its applets through /bin/ls pointing at /bin/busybox, and
//! libraries are found through their soname links. The image lists its links
//! in /etc/nonos-links, one `path target` a line, and resolution follows them
//! a component at a time. Every result passes through `visible` again, so a
//! link cannot point out of the guest's tree, and a program reached through
//! one is proved by its own path, never the link's.

use alloc::vec::Vec;

use crate::linux::file::visible;

/// Linux gives up at forty; a table this small never legitimately nears it.
const MAX_HOPS: usize = 16;

#[derive(Default)]
pub struct Links(pub(super) Vec<(Vec<u8>, Vec<u8>)>);

impl Links {
    /// `path` with every link in it followed; the last component too when
    /// `last` is set, which is everything but lstat, readlink and the *at
    /// calls that act on a name rather than what it names.
    pub fn follow(&self, mut path: Vec<u8>, last: bool) -> Vec<u8> {
        for _ in 0..MAX_HOPS {
            let Some((end, target)) = self.first_in(&path, last) else {
                return path;
            };
            let dir_end = path[..end].iter().rposition(|b| *b == b'/').unwrap_or(0);
            let mut joined = match target.first() == Some(&b'/') {
                true => Vec::new(),
                false => path[..dir_end].to_vec(),
            };
            joined.push(b'/');
            joined.extend_from_slice(target);
            joined.extend_from_slice(&path[end..]);
            path = visible(b"/", &joined);
        }
        path
    }

    /// The target of `path` itself, when it is a link.
    pub fn target(&self, path: &[u8]) -> Option<&[u8]> {
        self.0.iter().find(|(from, _)| from == path).map(|(_, to)| &to[..])
    }

    fn first_in(&self, path: &[u8], last: bool) -> Option<(usize, &[u8])> {
        let ends = path.iter().enumerate().skip(1).filter(|(_, b)| **b == b'/').map(|(i, _)| i);
        let whole = last.then_some(path.len());
        ends.chain(whole).find_map(|end| self.target(&path[..end]).map(|t| (end, t)))
    }
}

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

//! Listing a directory, the totals statfs reports, and closing the files
//! of clients that ended.

use alloc::vec::Vec;

use super::Broker;
use crate::errno::{Errno, ENOTDIR};
use crate::host::{Host, Objects};
use crate::limits::{LIST_MAX, MAX_NODES};
use crate::node::Kind;
use crate::path::parts;

/// One name a listing returns.
pub struct Entry {
    pub ino: u64,
    pub kind: Kind,
    pub name: Vec<u8>,
}

/// statfs: the quota and what is used of it, nodes and the most there may be.
pub struct Totals {
    pub quota: u64,
    pub used: u64,
    pub nodes: u64,
    pub max_nodes: u64,
}

impl<O: Objects> Broker<O> {
    /// Up to `max` names of the directory at `path`, in byte order, from
    /// the `cookie`-th; the next cookie, and whether that was the last.
    pub fn list(
        &mut self,
        path: &[u8],
        cookie: u64,
        max: usize,
        now: u64,
    ) -> Result<(Vec<Entry>, u64, bool), Errno> {
        let parts = parts(path)?;
        let (_, s) = self.ready(now)?;
        let dir = s.tree.get(s.tree.walk(&parts)?)?;
        if dir.kind != Kind::Dir {
            return Err(ENOTDIR);
        }
        let mut out = Vec::new();
        let names = dir.children.iter().skip(usize::try_from(cookie).unwrap_or(usize::MAX));
        for (name, ino) in names.take(max.min(LIST_MAX)) {
            let kind = s.tree.get(*ino)?.kind;
            out.push(Entry { ino: *ino, kind, name: name.clone() });
        }
        let next = cookie + out.len() as u64;
        Ok((out, next, next >= dir.children.len() as u64))
    }

    pub fn statfs(&mut self, now: u64) -> Result<Totals, Errno> {
        let quota = self.quota;
        let (_, s) = self.ready(now)?;
        let nodes = s.tree.nodes.len() as u64;
        Ok(Totals { quota, used: s.used, nodes, max_nodes: MAX_NODES as u64 })
    }

    /// Closes every handle of a client that no longer runs.
    pub fn reap(&mut self, host: &impl Host) {
        self.clock(host.mono_ns());
        let Some(s) = self.state.as_mut() else {
            return;
        };
        for owner in s.handles.owners().into_iter().filter(|o| !host.alive(*o)) {
            for h in s.handles.close_all(owner) {
                if let Ok(node) = s.tree.get_mut(h.ino) {
                    node.open = node.open.saturating_sub(1);
                }
                if h.wrote {
                    s.owe();
                }
                s.release(h.ino);
            }
        }
    }
}

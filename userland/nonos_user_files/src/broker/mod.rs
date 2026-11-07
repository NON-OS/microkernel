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

//! The broker: the tree, its chunks and its handles, and when it commits.
//!
//! A rename and an fsync commit before they answer. Every other change
//! owes a commit within GRACE_NS (`tick`), so a burst of creates is one
//! commit, not one each. A program's save, written to a temporary name and
//! renamed over the document, is therefore on the volume when the rename
//! answers, and a cut before that leaves the old document.

mod io;
mod names;
mod open;
mod rename;
mod view;

use alloc::vec::Vec;

use crate::chunks::{Dirty, Files};
use crate::errno::Errno;
use crate::handles::Handles;
use crate::host::Objects;
use crate::store::{self, Store};
use crate::tree::Tree;

/// How long a change waits for its commit at most.
pub const GRACE_NS: u64 = 500_000_000;

pub struct Broker<O: Objects> {
    objects: O,
    state: Option<State>,
    quota: u64,
    /// The monotonic clock at this request, which commits are timed by: a
    /// wall clock set back would hold an owed commit for as long.
    mono: u64,
}

pub(crate) struct State {
    pub tree: Tree,
    pub store: Store,
    pub dirty: Dirty,
    pub handles: Handles,
    /// Bytes every file holds, open-but-unlinked ones too.
    pub used: u64,
    /// When the owed commit is due, on the monotonic clock.
    pub due: Option<u64>,
    pub mono: u64,
}

impl<O: Objects> Broker<O> {
    /// A broker over `objects`. It loads the tree at its first request, and
    /// again at each later one until a load succeeds.
    pub fn new(objects: O, quota: u64) -> Broker<O> {
        Broker { objects, state: None, quota, mono: 0 }
    }

    pub(crate) fn ready(&mut self, now: u64) -> Result<(&mut O, &mut State), Errno> {
        if self.state.is_none() {
            let (mut tree, store) = store::load(&mut self.objects, now)?;
            // A tree committed before a folder existed gains it now.
            let grew = tree.ensure_folders(now);
            let used = tree.nodes.values().map(|n| n.size).sum();
            let (dirty, handles) = (Dirty::default(), Handles::default());
            let mono = self.mono;
            let mut state = State { tree, store, dirty, handles, used, due: None, mono };
            if grew {
                state.owe();
            }
            self.state = Some(state);
        }
        match self.state.as_mut() {
            Some(state) => Ok((&mut self.objects, state)),
            None => Err(crate::errno::EIO),
        }
    }

    /// Sets the monotonic clock the next calls are timed by.
    pub fn clock(&mut self, mono: u64) {
        self.mono = mono;
        if let Some(s) = self.state.as_mut() {
            s.mono = mono;
        }
    }

    /// Commits now if a commit is owed and due by the monotonic `mono`.
    pub fn tick(&mut self, mono: u64) -> Result<(), Errno> {
        self.clock(mono);
        let Some(state) = self.state.as_mut() else {
            return Ok(());
        };
        match state.due {
            Some(due) if due <= mono => commit(&mut self.objects, state),
            _ => Ok(()),
        }
    }

    /// When the owed commit is due on the monotonic clock, for the capsule's wait.
    pub fn due(&self) -> Option<u64> {
        self.state.as_ref().and_then(|s| s.due)
    }

    /// The objects underneath, for a host test to cut or inspect.
    pub fn objects(&mut self) -> &mut O {
        &mut self.objects
    }
}

impl State {
    pub fn files<'a, O: Objects>(&'a mut self, objects: &'a mut O) -> Files<'a, O> {
        Files { objects, tree: &mut self.tree, store: &mut self.store, dirty: &mut self.dirty }
    }

    pub fn owe(&mut self) {
        let by = self.mono.saturating_add(GRACE_NS);
        self.due = Some(self.due.map_or(by, |d| d.min(by)));
    }

    /// Drops a node nothing names and nothing holds open: its chunks wait
    /// for the next commit to be removed, which it owes.
    pub fn release(&mut self, ino: u64) {
        let gone = self.tree.nodes.get(&ino).is_some_and(|n| !n.linked && n.open == 0);
        if let Some(node) = gone.then(|| self.tree.nodes.remove(&ino)).flatten() {
            self.store.pending.extend(node.chunks.iter().filter(|id| **id != 0));
            self.dirty.forget(ino, 0);
            self.used = self.used.saturating_sub(node.size);
            self.owe();
        }
    }

    pub fn touch(&mut self, ino: u64, now: u64) {
        if let Some(n) = self.tree.nodes.get_mut(&ino) {
            (n.mtime, n.ctime) = (now, now);
        }
    }
}

/// Writes every changed chunk, then the tree.
pub(crate) fn commit<O: Objects>(objects: &mut O, s: &mut State) -> Result<(), Errno> {
    s.files(objects).flush_all(None)?;
    let unlinked = s.tree.nodes.values().filter(|n| !n.linked);
    let orphans: Vec<u64> =
        unlinked.flat_map(|n| n.chunks.iter().copied()).filter(|id| *id != 0).collect();
    store::commit(objects, &s.tree, &mut s.store, &orphans)?;
    s.due = None;
    Ok(())
}

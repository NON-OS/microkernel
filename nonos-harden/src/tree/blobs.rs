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

//! The receipt's commit, file by file (`git ls-tree`), held against the tree
//! on disk by this program's own hashing: each tracked path must hold the
//! blob git records, with its kind and executable bit, read once without
//! following a link. Git's filters, index flags and rename detection never
//! enter into it, so nothing in the tree's configuration can hide an edit.

use super::blob_same::same;
use super::git::git;
use super::untracked::untracked;
use std::collections::BTreeSet;
use std::path::Path;

pub struct Entry {
    pub mode: String,
    pub id: String,
    pub path: String,
}

pub fn tracked(tree: &Path, commit: &str) -> Result<Vec<Entry>, String> {
    let listing = git(tree, &["ls-tree", "-r", "-z", "--full-tree", commit])?;
    let entry = |e: &str| {
        let (meta, path) = e.split_once('\t')?;
        let mut m = meta.split(' ');
        let (mode, _kind, id) = (m.next()?, m.next()?, m.next()?);
        Some(Entry { mode: mode.into(), id: id.into(), path: path.into() })
    };
    let entries = listing.split('\0').filter(|e| !e.is_empty());
    entries.map(|e| entry(e).ok_or(format!("git listed `{e}`"))).collect()
}

// Every path, tracked or not, whose bytes on disk are not commit `c`'s.
pub fn differences(tree: &Path, c: &str) -> Result<Vec<String>, String> {
    let entries = tracked(tree, c)?;
    let mut changed = Vec::new();
    for e in &entries {
        if !same(tree, e)? {
            changed.push(e.path.clone());
        }
    }
    let known: BTreeSet<&str> = entries.iter().map(|e| e.path.as_str()).collect();
    changed.extend(untracked(tree, &known)?);
    Ok(changed)
}

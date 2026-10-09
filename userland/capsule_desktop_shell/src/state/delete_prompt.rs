// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Delete on the desktop asks first. Removing a file or a whole folder from
//! the right-click menu took it at once, one misplaced click from losing it,
//! with no Trash to get it back from. The menu now only asks; the delete
//! happens on the prompt's Delete button and on nothing else.
//!
//! The prompt keeps the item's name, not its place in the listing: the
//! desktop re-reads its directory while the prompt is up, and a position
//! could by then name a different file.

use alloc::string::String;

/// The desktop item a delete was asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeleteTarget {
    pub name: String,
    pub is_dir: bool,
}

/// Whether a delete is waiting on its answer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeletePrompt {
    target: Option<DeleteTarget>,
}

impl DeletePrompt {
    pub const fn new() -> Self {
        DeletePrompt { target: None }
    }

    /// Ask to delete `name`. A second ask replaces the first, unanswered one.
    pub fn ask(&mut self, name: &str, is_dir: bool) {
        self.target = Some(DeleteTarget { name: String::from(name), is_dir });
    }

    pub fn showing(&self) -> bool {
        self.target.is_some()
    }

    /// The item the prompt is asking about, while it is up.
    pub fn target(&self) -> Option<&DeleteTarget> {
        self.target.as_ref()
    }

    /// Close the prompt. Returns the item to delete only when `confirm`;
    /// any other answer deletes nothing.
    pub fn answer(&mut self, confirm: bool) -> Option<DeleteTarget> {
        let target = self.target.take()?;
        confirm.then_some(target)
    }
}

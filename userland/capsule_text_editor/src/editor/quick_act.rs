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

//! Finishing a quick open: what was typed, against the tree, into a document.
//!
//! The matching itself is in `quick_open` and knows nothing about the editor.
//! This is the part that does: reading the tree, opening the file, and saying
//! so when nothing matched.

use alloc::string::String;
use alloc::vec::Vec;
use nonos_app_skeleton::EventOutcome;

use super::app::Editor;
use super::quick_open::best;

impl Editor {
    /// Read the current document without borrowing it mutably, so a caller can
    /// look at the prompt before deciding to act on it.
    pub(super) fn doc_ref(&self) -> &super::state::State {
        &self.docs[self.active]
    }

    pub(super) fn finish_quick_open(&mut self) -> EventOutcome {
        let typed: Vec<u8> = {
            let d = self.doc_ref();
            d.prompt_path[..d.prompt_len].to_vec()
        };

        // Directories are not openable, so they are not offered. Searching
        // them would let a three-letter query land on a folder and do nothing.
        let files: Vec<String> =
            self.tree.nodes.iter().filter(|n| !n.is_dir).map(|n| n.path.clone()).collect();
        let refs: Vec<&[u8]> = files.iter().map(|p| p.as_bytes()).collect();

        let Some(hit) = best(&refs, &typed) else {
            let d = self.doc();
            d.prompt = None;
            d.prompt_len = 0;
            d.status = b"no file matches";
            return EventOutcome::Repaint;
        };

        let path = files[hit].clone();
        let bytes = path.as_bytes();
        let d = self.doc();
        d.prompt = None;
        d.prompt_len = 0;
        // The document's path is the commit point, exactly as the open prompt
        // does it, so the file loads through the one path that already handles
        // a read failing.
        let n = bytes.len().min(d.path.len());
        d.path[..n].copy_from_slice(&bytes[..n]);
        d.path_len = n;
        super::ctrl_open::ctrl_open(d)
    }
}

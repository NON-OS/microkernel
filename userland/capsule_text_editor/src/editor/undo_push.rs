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

//! Recording an edit on the undo stack, merging a typing run into one
//! step and dropping the oldest steps in batches.

use super::edit::EditOp;
use super::state::State;
use alloc::vec::Vec;

const MAX_UNDO: usize = 400;
/// Oldest steps are dropped in batches, so a long session does not shift
/// the whole stack on every keystroke once it is full.
const UNDO_SLACK: usize = 64;

impl State {
    /*
     * Push an undo step, coalescing a run of single-character typing so one
     * Ctrl-Z removes a word rather than a letter.
     */
    pub(super) fn push_undo(&mut self, at: usize, removed: Vec<u8>, inserted_len: usize) {
        self.saved.next_op = self.saved.next_op.wrapping_add(1).max(1);
        let id = self.saved.next_op;
        if removed.is_empty() && inserted_len == 1 {
            if let Some(last) = self.undo.last_mut() {
                /*
                 * A merged step is a new document state, so it takes the new
                 * id: a save made mid-word must not match the finished word.
                 */
                if last.deleted.is_empty() && last.at + last.inserted_len == at {
                    last.inserted_len += 1;
                    last.id = id;
                    return;
                }
            }
        }
        self.undo.push(EditOp { at, deleted: removed, inserted_len, id });
        if self.undo.len() > MAX_UNDO + UNDO_SLACK {
            let cut = self.undo.len() - MAX_UNDO;
            /*
             * Undo can no longer reach below the dropped steps, so the state
             * they led to becomes the bottom of history.
             */
            self.saved.floor = self.undo[cut - 1].id;
            self.undo.drain(..cut);
        }
    }
}

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

use super::types::PER_OWNER;
use super::WindowTable;

impl WindowTable {
    /// The popups over `owner_pid`'s window `window_id`, and the popups over
    /// those, parents before their children, into `out`; how many. The
    /// window itself is not among them. A parent and its popups are always
    /// one owner's, so the walk never leaves it.
    pub fn subtree(&self, owner_pid: u32, window_id: u32, out: &mut [u32]) -> usize {
        let mut n = 0;
        let mut next = 0;
        let mut parent = window_id;
        loop {
            for w in self.windows() {
                let child = w.owner_pid == owner_pid && w.parent == parent && w.is_popup();
                if child && n < out.len() && !out[..n].contains(&w.window_id) {
                    out[n] = w.window_id;
                    n += 1;
                }
            }
            if next >= n {
                return n;
            }
            parent = out[next];
            next += 1;
        }
    }

    /// Whether `window_id` is `root` or one of the popups over it.
    pub fn within(&self, owner_pid: u32, root: u32, window_id: u32) -> bool {
        let mut at = window_id;
        for _ in 0..=self.held_by(owner_pid) {
            if at == root {
                return true;
            }
            match self.find(owner_pid, at) {
                Some(w) if w.is_popup() => at = w.parent,
                _ => return false,
            }
        }
        false
    }

    /// Take out the popups over `owner_pid`'s window `window_id` and the
    /// popups over those, topmost first, and with `itself` the window too
    /// when it is a popup; their ids into `out` in the order taken out, and
    /// how many. A popup goes with the window it was opened over.
    pub fn remove_popups(
        &mut self,
        owner_pid: u32,
        window_id: u32,
        itself: bool,
        out: &mut [u32],
    ) -> usize {
        let mut popups = [0u32; PER_OWNER];
        let n = self.subtree(owner_pid, window_id, &mut popups);
        let mut k = 0;
        let own = itself && self.find(owner_pid, window_id).is_some_and(|w| w.is_popup());
        let last = own.then_some(window_id);
        for id in popups[..n].iter().rev().copied().chain(last) {
            if self.remove(owner_pid, id).is_some() && k < out.len() {
                out[k] = id;
                k += 1;
            }
        }
        k
    }
}

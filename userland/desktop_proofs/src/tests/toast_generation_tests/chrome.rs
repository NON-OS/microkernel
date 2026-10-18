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

//! The panel's painting, counted.

use crate::toast_state::toasts::ToastQueue;

/// The chrome as the shell paints it: it shows the queue as it was at the
/// generation it was last painted with.
pub(super) struct Chrome {
    pub(super) drawn: u32,
    pub(super) shown: usize,
    pub(super) paints: usize,
}

impl Chrome {
    pub(super) fn sync(&mut self, q: &ToastQueue) {
        if self.drawn != q.generation() {
            self.drawn = q.generation();
            self.shown = q.iter_live().count();
            self.paints += 1;
        }
    }
}

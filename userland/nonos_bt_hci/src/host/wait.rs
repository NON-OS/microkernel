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
//! Waiting for the event that ends a command answered by Command Status
//! (Vol 4, Part E, 7.7.15). The wait is bounded by the poll budget, as a
//! command's is. Events kept while a command was waited for are searched
//! first; every other event met is kept for the caller, in order.

use super::state::Host;
use crate::error::HciError;
use crate::transport::Transport;

impl<T: Transport> Host<T> {
    /// The first event `pick` turns into a value, taken out of the stream;
    /// `None` if the budget runs out first.
    pub(super) fn wait_for<R>(
        &mut self,
        mut pick: impl FnMut(&[u8]) -> Result<Option<R>, HciError>,
    ) -> Result<Option<R>, HciError> {
        for i in 0..self.queue.len() {
            if let Some(r) = pick(&self.queue[i])? {
                self.queue.remove(i);
                return Ok(Some(r));
            }
        }
        for _ in 0..self.budget {
            let Some(event) = self.t.poll_event()? else {
                self.t.pause();
                continue;
            };
            match pick(&event)? {
                Some(r) => return Ok(Some(r)),
                None => self.keep(event)?,
            }
        }
        Ok(None)
    }
}

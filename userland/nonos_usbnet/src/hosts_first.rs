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

//! The search that stops at the first host whose ports hold the device.

use super::hosts::Hosts;

impl<T> Hosts<T> {
    /// Visit each known host in order until `visit` gives a value.
    pub fn first<B>(&mut self, mut visit: impl FnMut(u32, &mut T) -> Option<B>) -> Option<B> {
        self.list.iter_mut().find_map(|h| visit(h.port?, &mut h.keep))
    }
}

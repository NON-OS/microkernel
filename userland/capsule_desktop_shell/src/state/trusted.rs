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

//! The shell's half of the trusted path: its chrome surface, which band it
//! is in, and the secure attention chord's state.

use super::attention::Attention;

#[derive(Clone, Copy, Debug, Default)]
pub struct Trusted {
    /// The chrome's shared surface, as registered at start (0 before).
    pub chrome_handle: u64,
    /// Whether the chrome is in the trusted band now.
    pub up: bool,
    pub attention: Attention,
}

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

//! What the personality is about to run.

use alloc::vec::Vec;

use super::origin::Origin;

pub struct Launch {
    /// The guest-visible path, which is also argv[0].
    pub path: Vec<u8>,
    pub bytes: Vec<u8>,
    pub origin: Origin,
    /// Arguments after argv[0].
    pub args: Vec<Vec<u8>>,
}

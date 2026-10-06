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

//! Where carried files come from: the running system's vfs, through
//! whichever client the installer has.

use alloc::string::String;
use alloc::vec::Vec;

pub trait CarrySource {
    /// Every path under `prefix`, as the vfs lists them: whole paths, a
    /// directory with a trailing slash, anything that merely starts with
    /// `prefix` included.
    fn list(&mut self, prefix: &str) -> Vec<String>;

    /// The whole file at `path`, `None` when it cannot be read.
    fn read(&mut self, path: &str) -> Option<Vec<u8>>;
}

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

//! A write the made trees refuse (/dev, /proc and /sys are mounted
//! read-only): refused here before the store is ever asked, and so counted
//! here against a box, as a refused write anywhere else is (root_write.rs).

pub fn refused_write(path: &[u8]) -> bool {
    let made = super::owns(path);
    if made {
        crate::linux::boxed::note(crate::linux::boxed::Kind::Write, path);
    }
    made
}

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

//! What one package put into the store, kept beside its program record so it
//! can be taken out again: each file it wrote, and each link it added to the
//! link table. One line each, `f <path>` or `l <path>`; a path holding a
//! newline cannot be written in it, and the link table refuses those too.
//! Pure, so capsule_linux_proofs holds the format and the removal rules.

mod format;
mod remove;

pub use format::{decode, encode, Manifest};
pub use remove::{drop_links, only_mine};

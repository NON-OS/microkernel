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

//! Each installed package's manifest (`manifest`), kept in the family's
//! records under `.files/<package>`. A package name never starts with a dot
//! (the kernel's `package_arg` refuses one), so no package's program record
//! can be mistaken for this directory. Outside every family's tree, where no
//! guest can rewrite what an uninstall would take out.

mod all;
mod one;

pub(super) use all::{others, settle};
pub(super) use one::{read, write};

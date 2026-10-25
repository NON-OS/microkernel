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

//! The ACPI namespace (section 5.3): one tree of named objects that every
//! loaded definition block adds to.

mod add;
mod alias_gone;
mod climb;
mod entry;
mod field_unit;
mod kind;
mod method_info;
mod note;
mod path;
mod predefined;
mod remove;
mod resolve;
mod tree;

pub use entry::{Entry, Id, Origin, Predefined, Seg, ROOT};
pub use field_unit::FieldUnit;
pub use kind::Kind;
pub use method_info::MethodInfo;
pub use note::{Note, NoteKind, Notes};
pub use path::{absolute, write_path};
pub use tree::Namespace;

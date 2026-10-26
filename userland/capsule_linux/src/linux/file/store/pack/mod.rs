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

//! Installed packages as packs (nonos_linux_pack): one store object a
//! package, its files served out of it. A package's pack lives beside the
//! Linux tree, at `/linux-packs/<package>.pack`, with the trailer this
//! machine minted over its index beside it.

mod list;
mod load;
mod paths;
mod put;
mod serve;
mod shelf;
mod swap;
mod table;

pub use list::list;
pub use put::{put, remove};
pub use serve::{read, stat};
pub use shelf::Held;

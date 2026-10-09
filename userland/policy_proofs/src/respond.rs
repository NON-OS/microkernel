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

//! `crate::server::respond`, as capsule_policy/src/server/respond.rs lays it
//! out. Its two children are named here: a file included by path finds its
//! children beside it, and these two live one directory down.

#[path = "../../capsule_policy/src/server/respond/err.rs"]
pub mod err;
#[path = "../../capsule_policy/src/server/respond/ok.rs"]
pub mod ok;

pub use err::err;
pub use ok::ok;

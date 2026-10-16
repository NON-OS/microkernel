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

//! The `ls -l` column modules, mounted by path, plus a `term` stand-in.

#[path = "../../../capsule_terminal/src/term/util/format_u64.rs"]
pub mod format_u64;

pub mod term {
    pub mod util {
        pub use crate::format_u64::format_u64;
    }
}

#[path = "../../../capsule_terminal/src/command/builtin/fs/ls_date.rs"]
pub mod ls_date;

#[path = "../../../capsule_terminal/src/command/builtin/fs/ls_dots.rs"]
pub mod ls_dots;

#[path = "../../../capsule_terminal/src/command/builtin/fs/ls_long.rs"]
pub mod ls_long;

#[path = "../../../capsule_terminal/src/command/builtin/fs/ls_num.rs"]
pub mod ls_num;

#[path = "../../../capsule_terminal/src/command/builtin/fs/ls_total.rs"]
pub mod ls_total;

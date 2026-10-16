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

use alloc::vec::Vec;

use super::target::Target;

#[path = "args_parse.rs"]
mod args_parse;

pub use args_parse::parse;

pub struct PullArgs {
    pub target: Target,
    pub dest: Vec<u8>,
    pub no_clobber: bool,
    pub skip_unchanged: bool,
    pub auth: Option<Vec<u8>>,
    pub headers: Vec<Vec<u8>>,
    pub verify: bool,
}

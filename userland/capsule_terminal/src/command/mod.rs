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

pub mod builtin;
pub mod dispatch;
pub mod host;
pub mod output;

// The tokenizer, flag tables and "did you mean" now live in the nonos_shell
// library; re-exported here at the paths the capsule has always used
// (`command::parse`, `command::flags`, `command::suggest`) so the builtins and
// dispatch compile unchanged.
pub use nonos_shell::{flags, parse, suggest};

pub use dispatch::{alias_expand, expand, run, split_program, Conn, Outcome, Stmt};
pub use parse::parse;

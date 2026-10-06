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

mod alias_expand;
mod exec;
mod expand;
mod filter;
mod outcome;
mod pipeline;
mod redirect;
mod run;
mod statements;
mod tool_admit;
mod write_redirect;

pub use alias_expand::alias_expand;
pub use expand::expand;
pub(crate) use filter::count::{wc_row, word_count};
pub(crate) use filter::input::lines_of;
pub(crate) use filter::{apply as apply_filter, input::spec_for, input::without};
pub use outcome::Outcome;
pub(crate) use pipeline::{is_filter, run_stage, split_stages};
pub(crate) use redirect::{is_operator, plan as redirect_plan, Sink, Source};
pub use run::run;
pub use statements::{split_program, Conn, Stmt};
pub(crate) use tool_admit::{admit as admit_tool, hears_end_of_input};
pub(crate) use write_redirect::{not_written, write_out, wrote_to, REDIRECT_MAX};

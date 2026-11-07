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

//! NONOS shell: the terminal command language as a host-agnostic library.
//!
//! This crate holds the parts of the NONOS terminal that have no GUI and no
//! kernel coupling: turning a typed line into words, statements and redirects,
//! expanding `$NAME`/`$?`/aliases, routing a name to its command group, and the
//! pipe filters (`grep`, `sort`, `wc`, ...). The local GUI terminal
//! (`capsule_terminal`) and a headless `nsh` session consume the same code, so
//! the shell a person drives over a remote link is byte-for-byte the shell they
//! drive at the glass.
//!
//! The one environment coupling a caller supplies is an output sink,
//! [`out::ShellOut`]: the terminal implements it over its scrollback, a headless
//! session over its stream. See `README.md` for the headless entry contract and
//! for which capabilities each (still capsule-side) builtin needs.

#![no_std]

extern crate alloc;

/// Leaf byte/number helpers the language core needs (space test, `u64` to
/// decimal). A private copy of the two pure `const fn`s the capsule keeps in
/// `term::util`, so the library depends on nothing.
pub mod util;

/// The output sink a caller plugs in: the trait builtins write their lines to.
pub mod out;

/// What a builtin may ask of the machine: randomness, clocks, memory, files.
pub mod host;

/// The builtins that need nothing of the terminal, shared with nsh.
pub mod builtin;

/// A headless session: lines run through control flow, builtins and filters.
pub mod session;

/// Tokenize a line into words, honouring quotes.
pub mod parse;

/// The flag tables (`-n`, clustered short flags, `--`) the filters and builtins
/// parse their arguments with.
pub mod flags;

/// `$NAME` and `$?` substitution over a caller-supplied variable set and status.
pub mod expand;

/// Alias substitution over a caller-supplied alias set.
pub mod alias_expand;

/// Split a line into statements on `;`, `&&`, `||`, `&`, quote-aware.
pub mod statements;

/// The redirect plan (`<`, `>`, `>>`, `2>&1`) a dispatch and a tool's job read.
pub mod redirect;

/// Whether a tool is admitted to run given its redirect plan.
pub mod tool_admit;

/// "did you mean" nearest-command suggestion.
pub mod suggest;

/// The pipe filters: `grep`, `sort`, `uniq`, `cut`, `nl`, `wc`, `head`, `tail`,
/// `tac`, `rev`, over a pipe, a redirected file, or named files.
pub mod filter;

/// Which shell command each name means: the four group tables (Info, Session,
/// Files, Net) and the lookup across them.
pub mod route;

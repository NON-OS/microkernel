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

//! A headless shell session: what an nsh session runs a typed line through.
//! A line is split into statements (`;`, `&&`, `||`), read into its
//! control flow (`if`/`then`/`else`/`fi`, `for`/`in`/`do`/`done`,
//! `while`/`do`/`done`), and each simple command is alias- and
//! `$NAME`-expanded only when it runs, so a loop's variable is read afresh
//! each round. A command runs the shared builtins or, after a `|`, the
//! shared pipe filters. Every line is held to `MAX_STEPS` commands and every
//! `while` to `MAX_ROUNDS` rounds, so no line runs without end.

mod history;
mod names;
mod release;
mod run;
mod script;
mod simple;
mod table;
mod who;

use crate::host::Host;
use crate::out::ShellOut;
use alloc::vec::Vec;

pub use history::{MAX_HISTORY, MAX_HISTORY_LINE};
pub use release::release_line;
pub use names::{ALIAS_REFUSED, CONTROL, FILTERS, MAX_ALIASES};
pub use script::{parse, Node, Script};
pub use table::{CLEAR, NAMES};
pub use who::{Who, NO_LOGIN};

/// The most simple commands one line may run, loops included.
pub const MAX_STEPS: u32 = 100_000;
/// The most rounds one `while` may run.
pub const MAX_ROUNDS: u32 = 10_000;
/// The longest value a variable may hold. A value is built from the words of
/// one line, so without this a loop such as `set A $A$A` would double it each
/// round.
pub const MAX_VALUE: usize = 1024;
/// The most variables one session may hold.
pub const MAX_VARS: usize = 256;
/// Said when a variable would pass either bound; the variable keeps its
/// value.
pub const VAR_REFUSED: &[u8] =
    b"shell: a variable is held to 1024 bytes and a session to 256 of them";

/// What the caller does after a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    /// Read the next line.
    Next,
    /// The session asked to end, with this status.
    Exit(i32),
}

#[derive(Default)]
pub struct Session {
    pub vars: Vec<(Vec<u8>, Vec<u8>)>,
    pub aliases: Vec<(Vec<u8>, Vec<u8>)>,
    pub last_status: i32,
    /// The login this session runs as, if something admitted one.
    pub who: Option<Who>,
    /// The lines this session has run, newest last.
    pub history: Vec<Vec<u8>>,
}

/// One line's run: the session, where its output goes, the machine, and the
/// steps left.
pub(crate) struct Run<'a> {
    pub s: &'a mut Session,
    pub out: &'a mut dyn ShellOut,
    pub host: &'a mut dyn Host,
    pub steps: u32,
    /// The line ran out of steps and was told so.
    pub stopped: bool,
    pub exit: Option<i32>,
}

impl Session {
    /// Run one typed line to its end.
    pub fn run_line(&mut self, line: &[u8], out: &mut dyn ShellOut, host: &mut dyn Host) -> Flow {
        self.remember(line);
        let script = match parse(line) {
            Ok(s) => s,
            Err(msg) => {
                out.writeln(msg);
                self.last_status = 2;
                return Flow::Next;
            }
        };
        let mut r = Run { s: self, out, host, steps: MAX_STEPS, stopped: false, exit: None };
        let status = r.block(&script.0);
        let exit = r.exit;
        self.last_status = status;
        match exit {
            Some(code) => Flow::Exit(code),
            None => Flow::Next,
        }
    }

    /// The value of `name`, if it is set.
    pub fn var(&self, name: &[u8]) -> Option<&[u8]> {
        self.vars.iter().find(|(n, _)| n == name).map(|(_, v)| &v[..])
    }

    /// Set `name` to `value`; false, with nothing changed, when the value
    /// passes `MAX_VALUE` or a new name would pass `MAX_VARS`.
    pub fn set_var(&mut self, name: &[u8], value: &[u8]) -> bool {
        if value.len() > MAX_VALUE {
            return false;
        }
        let full = self.vars.len() >= MAX_VARS;
        match self.vars.iter_mut().find(|(n, _)| n == name) {
            Some((_, v)) => *v = value.to_vec(),
            None if full => return false,
            None => self.vars.push((name.to_vec(), value.to_vec())),
        }
        true
    }
}

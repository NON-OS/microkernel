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

//! One simple command: its aliases and `$NAME`s expanded as it runs, its
//! words read, and either one builtin run or, through `|`, a builtin whose
//! lines then pass the shared pipe filters in turn. File redirects need a
//! filesystem this shell does not hold and are refused.

use super::table::{call, Done};
use super::Run;
use crate::alias_expand::alias_expand;
use crate::expand::expand;
use crate::filter::apply;
use crate::out::ShellOut;
use crate::parse::{parse, TOO_MANY};
use crate::redirect::is_operator;
use alloc::vec::Vec;

/// A sink that keeps a stage's lines for the next.
struct Kept(Vec<Vec<u8>>);

impl ShellOut for Kept {
    fn writeln(&mut self, line: &[u8]) {
        self.0.push(line.to_vec());
    }
}

impl Run<'_> {
    pub(super) fn simple(&mut self, body: &[u8]) -> i32 {
        if !self.tick() {
            return 1;
        }
        let aliased = alias_expand(body, &self.s.aliases);
        let line = expand(&aliased, &self.s.vars, self.s.last_status);
        let argv = parse(&line);
        if argv.cut {
            self.out.writeln(TOO_MANY);
            return 1;
        }
        let words = &argv.argv[..argv.argc];
        if words.iter().any(|w| is_operator(w) && *w != b"|") {
            self.out.writeln(b"redirects are not available in this shell");
            return 1;
        }
        let mut stages = words.split(|w| *w == b"|");
        let first = stages.next().unwrap_or(&[]);
        let rest: Vec<&[&[u8]]> = stages.collect();
        if rest.is_empty() {
            let d = call(self.s, &mut *self.out, &mut *self.host, first);
            return self.done(d);
        }
        let mut kept = Kept(Vec::new());
        let d = call(self.s, &mut kept, &mut *self.host, first);
        let status = self.done(d);
        // Every row a pipe carries is a step, so filters over a builtin's
        // rows cost the line what printing them would.
        let rows = u32::try_from(kept.0.len()).unwrap_or(u32::MAX);
        self.steps = self.steps.saturating_sub(rows);
        let lines = rest.iter().fold(kept.0, |lines, stage| apply(stage, lines));
        lines.iter().for_each(|l| self.out.writeln(l));
        status
    }

    /// Take one step; false, having taken none, when they are spent.
    pub(super) fn tick(&mut self) -> bool {
        if self.spent() {
            return false;
        }
        self.steps -= 1;
        true
    }

    /// Whether the line's steps are spent, said once when they are, or its
    /// output is full, which the sink has said.
    pub(super) fn spent(&mut self) -> bool {
        if self.out.full() {
            return true;
        }
        if self.steps > 0 {
            return false;
        }
        if !self.stopped {
            self.stopped = true;
            self.out.writeln(b"shell: stopped after 100000 commands on one line");
        }
        true
    }

    fn done(&mut self, d: Done) -> i32 {
        match d {
            Done::Status(s) => s,
            Done::Exit(code) => {
                self.exit = Some(code);
                code
            }
        }
    }
}

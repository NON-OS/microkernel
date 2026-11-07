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

//! Running a line's blocks. Each node is gated on the status before it by
//! its connector; a block's status is its last command's. An `if` with no
//! branch taken is 0. `for` sets its name to each word in turn. `while`
//! stops when its condition fails, after `MAX_ROUNDS` rounds, or when the
//! line's steps run out; the last two fail. Every loop round is charged a
//! step, so a loop with nothing in it is held to the line's steps too.

use super::script::Node;
use super::{Run, MAX_ROUNDS, VAR_REFUSED};
use crate::expand::expand;
use crate::parse::{parse, TOO_MANY};
use crate::statements::Conn;

impl Run<'_> {
    pub(super) fn block(&mut self, nodes: &[(Conn, Node<'_>)]) -> i32 {
        let mut status = self.s.last_status;
        for (conn, node) in nodes {
            if self.exit.is_some() {
                break;
            }
            let go = match conn {
                Conn::Always => true,
                Conn::And => status == 0,
                Conn::Or => status != 0,
            };
            if go {
                status = self.node(node);
                self.s.last_status = status;
            }
        }
        status
    }

    fn node(&mut self, node: &Node<'_>) -> i32 {
        match node {
            Node::Cmd(body) => self.simple(body),
            Node::If { cond, then, els } => match self.block(cond) {
                0 => self.block(then),
                _ if els.is_empty() => 0,
                _ => self.block(els),
            },
            Node::For { name, words, body } => self.each(name, words, body),
            Node::While { cond, body } => self.rounds(cond, body),
        }
    }

    fn each(&mut self, name: &[u8], words: &[u8], body: &[(Conn, Node<'_>)]) -> i32 {
        let line = expand(words, &self.s.vars, self.s.last_status);
        let argv = parse(&line);
        if argv.cut {
            self.out.writeln(TOO_MANY);
            return 1;
        }
        let mut status = 0;
        for w in &argv.argv[..argv.argc] {
            if self.exit.is_some() {
                break;
            }
            if !self.tick() {
                return 1;
            }
            if !self.s.set_var(name, w) {
                self.out.writeln(VAR_REFUSED);
                return 1;
            }
            status = self.block(body);
        }
        status
    }

    fn rounds(&mut self, cond: &[(Conn, Node<'_>)], body: &[(Conn, Node<'_>)]) -> i32 {
        let mut status = 0;
        for _ in 0..MAX_ROUNDS {
            if self.block(cond) != 0 || self.exit.is_some() {
                return status;
            }
            if !self.tick() {
                return 1;
            }
            status = self.block(body);
        }
        self.out.writeln(b"while: stopped after 10000 rounds");
        1
    }
}

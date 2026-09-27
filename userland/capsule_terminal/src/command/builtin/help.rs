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

//! `help`: the commands, grouped, and the way to each deeper page. Keys and
//! shell syntax were in the same wall once, which filled the window and pushed
//! the command that asked for it out of sight; now `help keys`, `help shell`.

use crate::command::output::Output;

const GROUPS: &[(&[u8], &[u8])] = &[
    (b"files", b"ls  tree  cat  cd  pwd  mkdir  touch  rm  rmdir  mv  cp  stat  find  du"),
    (b"text", b"head  tail  grep  wc  echo  sort  uniq  cut  nl  tac  rev"),
    (b"system", b"capsules  service  ps  kill  sys  id  whoami  date  uptime  battery  about"),
    (b"net", b"ping  ifconfig  nslookup  curl  nym"),
    (b"apps", b"market  install  pkg  git  nox"),
];

const DEEPER: &[(&[u8], &[u8])] = &[
    (b"help keys", b"editing, history, tabs and view keys"),
    (b"help shell", b"pipes, redirects, jobs and aliases"),
    (b"help <cmd>", b"what one command takes"),
];

pub fn run(out: &mut Output<'_>) {
    out.writeln(b"Type a command and press Enter. Tab completes.");
    out.writeln(b"");
    for (name, list) in GROUPS {
        row(out, name, list, 9);
    }
    super::help_tools::tools(out);
    out.writeln(b"");
    for (name, what) in DEEPER {
        row(out, name, what, 13);
    }
}

/// `help keys` and `help shell`; false for anything else.
pub fn topic(out: &mut Output<'_>, name: &[u8]) -> bool {
    let lines: &[(&[u8], &[u8])] = match name {
        b"keys" => &super::help_pages::KEYS,
        b"shell" => &super::help_pages::SHELL,
        _ => return false,
    };
    for (name, what) in lines {
        row(out, name, what, 10);
    }
    true
}

/// A label in the accent colour, padded to `pad`, then the text.
fn row(out: &mut Output<'_>, name: &[u8], text: &[u8], pad: usize) {
    let mut plain = alloc::vec![b' '; 2];
    plain.extend_from_slice(name);
    plain.resize(2 + pad.max(name.len() + 1), b' ');
    plain.extend_from_slice(text);
    let mut styled = alloc::vec::Vec::with_capacity(plain.len() + 12);
    styled.extend_from_slice(b"  \x1b[36m");
    styled.extend_from_slice(name);
    styled.extend_from_slice(b"\x1b[0m");
    styled.extend_from_slice(&plain[2 + name.len()..]);
    out.writeln_styled(&plain, &styled);
}

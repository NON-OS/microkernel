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

/*
 * What `help` shows and how its rows are laid out, with nothing else in the
 * file, so the proofs render the same rows the terminal prints.
 */

use alloc::vec::Vec;

/// The width `help` has to fit: what every terminal opens at.
pub const WIDTH: usize = 80;
/// Where the text column starts, after the label, in each table.
pub const GROUP_PAD: usize = 9;
pub const DEEPER_PAD: usize = 13;
pub const PAGE_PAD: usize = 10;

pub const INTRO: &[u8] = b"Type a command and press Enter. Tab completes.";

pub const GROUPS: &[(&[u8], &[u8])] = &[
    (b"files", b"ls  tree  cat  cd  pwd  mkdir  touch  rm  rmdir  mv  cp"),
    (b"disk", b"stat  find  du"),
    (b"text", b"head  tail  grep  wc  echo  sort  uniq  cut  nl  tac  rev"),
    (b"system", b"capsules  service  ps  kill  sys  battery  about"),
    (b"session", b"id  whoami  date  uptime"),
    (b"net", b"ping  ifconfig  nslookup  curl  nym"),
    (b"apps", b"market  install  pkg  git  nox"),
];

pub const DEEPER: &[(&[u8], &[u8])] = &[
    (b"help keys", b"editing, history, tabs, selection and search keys"),
    (b"help shell", b"pipes, redirects, jobs and aliases"),
    (b"help <cmd>", b"what one command takes"),
];

/// A row as plain text: indent, the label padded to `pad`, then the text.
pub fn plain_row(name: &[u8], text: &[u8], pad: usize) -> Vec<u8> {
    let mut plain = alloc::vec![b' '; 2];
    plain.extend_from_slice(name);
    plain.resize(2 + pad.max(name.len() + 1), b' ');
    plain.extend_from_slice(text);
    plain
}

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

//! The rows `neofetch` prints beside the logo: who, which system, how long up.

use alloc::vec::Vec;

use crate::term::identity::{hostname, username};

fn row(label: &str, value: &[u8]) -> Vec<u8> {
    let mut line = Vec::with_capacity(48);
    line.extend_from_slice(label.as_bytes());
    while line.len() < 8 {
        line.push(b' ');
    }
    line.extend_from_slice(value);
    line
}

pub(super) fn info(kernel: &[u8], up: &[u8], signed: &[u8]) -> Vec<Vec<u8>> {
    let mut head = Vec::with_capacity(32);
    head.extend_from_slice(username());
    head.push(b'@');
    head.extend_from_slice(hostname());
    let mut rule = Vec::with_capacity(head.len());
    rule.resize(head.len(), b'-');
    alloc::vec![
        head,
        rule,
        Vec::from(&b"ZeroState Cryptographic OS"[..]),
        Vec::new(),
        row("os", crate::paint::fetch_boot::os_line().as_bytes()),
        row("kernel", kernel),
        row("shell", b"nox   (type 'help')"),
        row("signed", signed),
        row("arch", b"x86_64"),
        row("uptime", up),
    ]
}

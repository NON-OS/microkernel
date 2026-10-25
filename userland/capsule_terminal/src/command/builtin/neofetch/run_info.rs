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

//! The rows `neofetch` prints beside the logo, each as plain text and as the
//! same text with its label in the accent, as fastfetch draws it.

use alloc::vec::Vec;

use crate::term::identity::{hostname, username};

use super::super::compose::Row;

/// What the rows say, every figure read from the system (`gather`).
pub struct Facts<'a> {
    pub kernel: &'a [u8],
    pub uptime: &'a [u8],
    pub signed: &'a [u8],
    pub memory: &'a [u8],
    pub processes: &'a [u8],
    pub load: &'a [u8],
    pub terminal: &'a [u8],
    pub theme: &'a [u8],
}

const KEY: &[u8] = b"\x1b[1;36m";
const OFF: &[u8] = b"\x1b[0m";

fn row(label: &str, value: &[u8]) -> Row {
    let pad = 10usize.saturating_sub(label.len());
    let (mut plain, mut styled) = (Vec::new(), Vec::new());
    plain.extend_from_slice(label.as_bytes());
    styled.extend_from_slice(KEY);
    styled.extend_from_slice(label.as_bytes());
    styled.extend_from_slice(OFF);
    for out in [&mut plain, &mut styled] {
        out.resize(out.len() + pad, b' ');
        out.extend_from_slice(value);
    }
    (plain, styled)
}

pub(super) fn info(f: &Facts) -> Vec<Row> {
    let (user, host) = (username(), hostname());
    let head: Vec<u8> = [user, b"@", host].concat();
    let styled: Vec<u8> = [KEY, user, OFF, b"@", KEY, host, OFF].concat();
    alloc::vec![
        (head.clone(), styled),
        (alloc::vec![b'-'; head.len()], alloc::vec![b'-'; head.len()]),
        row("os", crate::paint::fetch_boot::os_line().as_bytes()),
        row("kernel", f.kernel),
        row("arch", crate::paint::fetch_arch::arch().as_bytes()),
        row("signed", f.signed),
        row("uptime", f.uptime),
        row("memory", f.memory),
        row("processes", f.processes),
        row("load", f.load),
        row("shell", b"nox"),
        row("terminal", f.terminal),
        row("theme", f.theme),
    ]
}

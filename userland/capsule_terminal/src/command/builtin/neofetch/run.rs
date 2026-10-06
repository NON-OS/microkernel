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

//! `neofetch` — the fresh-tab splash rendered as scrollback text.

use alloc::vec::Vec;
use nonos_libc::mk_uptime_ms;

use crate::command::output::Output;
use crate::term::identity::{hostname, username};
use crate::term::state::State;
use crate::term::util::copy_into;

use super::compose::two_column;
use super::logo::LOGO;
use super::palette::palette;

const VERSION: &str = include_str!("../../../../../../VERSION");

const GAP: usize = 2;

fn row(label: &str, value: &[u8]) -> Vec<u8> {
    let mut line = Vec::with_capacity(48);
    line.extend_from_slice(label.as_bytes());
    while line.len() < 8 {
        line.push(b' ');
    }
    line.extend_from_slice(value);
    line
}

/*
 * The system's uptime, from the monotonic clock `uptime` reads. It was the
 * time since this tab opened, under a row labelled uptime.
 */
fn uptime(buf: &mut [u8]) -> usize {
    let ms = mk_uptime_ms();
    if ms < 0 {
        return copy_into(buf, b"unavailable");
    }
    crate::paint::fetch_uptime::uptime_str(ms as u64, buf)
}

fn info(kernel: &[u8], up: &[u8], signed: &[u8]) -> Vec<Vec<u8>> {
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

pub fn run(state: &mut State) {
    let mut ubuf = [0u8; 32];
    let n = uptime(&mut ubuf);
    let signed = crate::command::builtin::receipt::own_line();
    let mut kernel = Vec::with_capacity(32);
    kernel.extend_from_slice(b"microkernel ");
    kernel.extend_from_slice(VERSION.trim_end().as_bytes());

    let rows = two_column(&LOGO, &info(&kernel, &ubuf[..n], signed), GAP);
    let (plain, styled) = palette();

    let out = &mut Output::new(&mut state.scrollback);
    for line in &rows {
        out.writeln(line);
    }
    out.writeln(b"");
    out.writeln_styled(&plain, &styled);
}

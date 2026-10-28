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

//! The line discipline on a byte the master side writes, as Linux's n_tty
//! receives it: input mapping, then a signal under ISIG, then the byte read
//! raw or edited into a line under ICANON, echoed as the settings say. Pure.

use alloc::collections::VecDeque;

use super::flags::{cc, iflag, lflag, Termios, ICANON, ICRNL, IGNCR, INLCR, ISIG, ISTRIP};
use super::flags::{NOFLSH, VQUIT, VSUSP};
use super::input::Input;
use super::out::echo;
use crate::linux::console::tty_rules::interrupt_byte;

pub const SIGINT: u8 = 2;
pub const SIGQUIT: u8 = 3;
pub const SIGTSTP: u8 = 20;

/// The signal byte `c` raises under ISIG: VINTR, VQUIT or VSUSP.
fn signal_of(t: &Termios, c: u8) -> Option<u8> {
    match Some(c) {
        _ if interrupt_byte(t, true) == Some(c) => Some(SIGINT),
        q if q == cc(t, VQUIT) => Some(SIGQUIT),
        s if s == cc(t, VSUSP) => Some(SIGTSTP),
        _ => None,
    }
}

/// Byte `c` from the master: what the slave may read and what is echoed
/// change, and the signal it raises for the foreground group is returned.
/// A signal drops what was typed and not read unless NOFLSH is set.
pub fn receive(t: &Termios, input: &mut Input, to: &mut VecDeque<u8>, c: u8) -> Option<u8> {
    let (i, l) = (iflag(t), lflag(t));
    let c = if i & ISTRIP != 0 { c & 0x7f } else { c };
    let c = match c {
        b'\r' if i & IGNCR != 0 => return None,
        b'\r' if i & ICRNL != 0 => b'\n',
        b'\n' if i & INLCR != 0 => b'\r',
        _ => c,
    };
    if let Some(sig) = signal_of(t, c).filter(|_| l & ISIG != 0) {
        if l & NOFLSH == 0 {
            *input = Input::default();
        }
        echo(t, c, to);
        return Some(sig);
    }
    match l & ICANON != 0 {
        true => super::edit::edit(t, input, to, c),
        false if input.full() => {}
        false => {
            input.ready.push_back(c);
            echo(t, c, to);
        }
    }
    None
}

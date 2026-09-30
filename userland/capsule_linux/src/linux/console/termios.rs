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

//! The terminal's settings as a guest reads and sets them: the kernel's
//! `struct termios`, starting as Linux's cooked defaults. They are kept
//! and given back; the terminal itself edits and echoes each line.

use super::state::with;

/// c_iflag, c_oflag, c_cflag and c_lflag, c_line, and 19 control bytes.
pub const SIZE: usize = 36;

pub type Termios = [u8; SIZE];

/// ICRNL | IXON.
const IFLAG: u32 = 0o2400;
/// OPOST | ONLCR.
const OFLAG: u32 = 0o5;
/// B38400 | CS8 | CREAD | HUPCL.
const CFLAG: u32 = 0o2277;
/// ISIG ICANON ECHO ECHOE ECHOK ECHOCTL ECHOKE IEXTEN.
const LFLAG: u32 = 0o105073;

const fn cooked() -> Termios {
    let mut t = [0u8; SIZE];
    let flags = [IFLAG, OFLAG, CFLAG, LFLAG];
    let mut i = 0;
    while i < 4 {
        let b = flags[i].to_le_bytes();
        t[i * 4] = b[0];
        t[i * 4 + 1] = b[1];
        t[i * 4 + 2] = b[2];
        t[i * 4 + 3] = b[3];
        i += 1;
    }
    /*
     * VINTR ^C, VQUIT ^\, VERASE DEL, VKILL ^U, VEOF ^D, VTIME 0, VMIN 1, VSWTC,
     * then VSTART, VSTOP, VSUSP, VEOL, VREPRINT, VDISCARD, VWERASE, VLNEXT.
     */
    let cc: [u8; 17] =
        [3, 0x1c, 0x7f, 0x15, 4, 0, 1, 0, 0x11, 0x13, 0x1a, 0, 0x12, 0xf, 0x17, 0x16, 0];
    let mut k = 0;
    while k < cc.len() {
        t[17 + k] = cc[k];
        k += 1;
    }
    t
}

pub(super) const COOKED: Termios = cooked();

/// The settings as they stand.
pub fn termios() -> Termios {
    with(|c| c.termios)
}

/// New settings, kept as given.
pub fn set_termios(raw: &[u8]) {
    if let Ok(t) = <Termios>::try_from(raw) {
        with(|c| c.termios = t);
    }
}

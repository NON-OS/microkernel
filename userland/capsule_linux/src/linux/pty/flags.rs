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

//! Where a setting sits in the kernel's `struct termios`, and the bits a
//! pseudo-terminal's line discipline reads. Pure, so the host proofs hold it.

pub use crate::linux::console::termios::{Termios, COOKED};

/// ICRNL, INLCR, IGNCR and ISTRIP in c_iflag.
pub const ICRNL: u32 = 0o400;
pub const INLCR: u32 = 0o100;
pub const IGNCR: u32 = 0o200;
pub const ISTRIP: u32 = 0o40;
/// OPOST, ONLCR and OCRNL in c_oflag.
pub const OPOST: u32 = 0o1;
pub const ONLCR: u32 = 0o4;
pub const OCRNL: u32 = 0o10;
/// ISIG ICANON ECHO ECHOE ECHONL NOFLSH ECHOCTL IEXTEN in c_lflag.
pub const ISIG: u32 = 0o1;
pub const ICANON: u32 = 0o2;
pub const ECHO: u32 = 0o10;
pub const ECHOE: u32 = 0o20;
pub const ECHONL: u32 = 0o100;
pub const NOFLSH: u32 = 0o200;
pub const ECHOCTL: u32 = 0o1000;
pub const IEXTEN: u32 = 0o100000;

/// Indexes into c_cc, which starts after c_line; VINTR is tty_rules'.
pub const VQUIT: usize = 1;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VEOF: usize = 4;
pub const VMIN: usize = 6;
pub const VSUSP: usize = 10;
pub const VEOL: usize = 11;
pub const VWERASE: usize = 14;

/// c_iflag, c_oflag, c_lflag: the words at 0, 4 and 12.
pub fn iflag(t: &Termios) -> u32 {
    word(t, 0)
}
pub fn oflag(t: &Termios) -> u32 {
    word(t, 4)
}
pub fn lflag(t: &Termios) -> u32 {
    word(t, 12)
}

/// Control byte `i`; 0 is _POSIX_VDISABLE, which names no byte.
pub fn cc(t: &Termios, i: usize) -> Option<u8> {
    t.get(17 + i).copied().filter(|&b| b != 0)
}

fn word(t: &Termios, at: usize) -> u32 {
    u32::from_le_bytes([t[at], t[at + 1], t[at + 2], t[at + 3]])
}

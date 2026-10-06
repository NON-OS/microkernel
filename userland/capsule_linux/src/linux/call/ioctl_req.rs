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

//! Which ioctl requests a descriptor answers. The file requests are any
//! descriptor's; the terminal's are a console's, and only while the family
//! is on a terminal; everything else is ENOTTY, which is how a program
//! learns a descriptor is not a terminal. Pure, so the host proofs hold the
//! table for every kind of descriptor.

use crate::linux::abi::errno;
use crate::linux::guest::Kind;

const TCGETS: u64 = 0x5401;
const TCSETS: u64 = 0x5402;
const TCSETSW: u64 = 0x5403;
const TCSETSF: u64 = 0x5404;
const TCSBRK: u64 = 0x5409;
const TCXONC: u64 = 0x540A;
const TCFLSH: u64 = 0x540B;
const TIOCGPGRP: u64 = 0x540F;
const TIOCSPGRP: u64 = 0x5410;
const TIOCOUTQ: u64 = 0x5411;
const TIOCGWINSZ: u64 = 0x5413;
const TIOCSWINSZ: u64 = 0x5414;
const FIONREAD: u64 = 0x541B;
const FIONBIO: u64 = 0x5421;
const TCSBRKP: u64 = 0x5425;
const TIOCGSID: u64 = 0x5429;
const FIONCLEX: u64 = 0x5450;
const FIOCLEX: u64 = 0x5451;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Req {
    /// FIOCLEX and FIONCLEX: close-on-exec set, or cleared.
    CloseOnExec(bool),
    /// FIONBIO: O_NONBLOCK from the int at the argument.
    NonBlock,
    /// FIONREAD: the bytes a read would find now.
    Queued,
    /// TCGETS.
    GetTermios,
    /// TCSETS and TCSETSW, and TCSETSF, which drops typed input first.
    SetTermios {
        flush: bool,
    },
    /// TIOCGWINSZ and TIOCSWINSZ.
    GetWinsize,
    SetWinsize,
    /// TIOCGPGRP and TIOCGSID: the foreground group and the session.
    Pgrp,
    Sid,
    /// TIOCSPGRP: make a group the foreground one. Every group of the
    /// family is in the foreground already, so it is taken as asked; job
    /// control in a shell (ash's setjobctl, tcsetpgrp) stops short without it.
    SetPgrp,
    /// TIOCOUTQ: written and not yet sent, which is nothing: the terminal
    /// takes each write whole.
    OutQueued,
    /// TCFLSH, with its queue selector.
    Flush,
    /// TCSBRK and TCSBRKP, which is tcdrain and tcsendbreak, and TCXONC:
    /// nothing is held back to wait for or to stop.
    Drain,
    FlowControl,
    /// ENOTTY.
    NotTty,
}

/// What `request` on a descriptor of `kind` is. A console answers the
/// terminal's requests only while `on_terminal`; off one, it is the pipe
/// /proc shows, and ENOTTY says so.
pub fn request(kind: Kind, on_terminal: bool, request: u64) -> Req {
    let console = on_terminal && matches!(kind, Kind::Stdin | Kind::Stdout | Kind::Stderr);
    match request & 0xFFFF_FFFF {
        FIOCLEX => Req::CloseOnExec(true),
        FIONCLEX => Req::CloseOnExec(false),
        FIONBIO => Req::NonBlock,
        FIONREAD => Req::Queued,
        _ if !console => Req::NotTty,
        TCGETS => Req::GetTermios,
        TCSETS | TCSETSW => Req::SetTermios { flush: false },
        TCSETSF => Req::SetTermios { flush: true },
        TIOCGWINSZ => Req::GetWinsize,
        TIOCSWINSZ => Req::SetWinsize,
        TIOCGPGRP => Req::Pgrp,
        TIOCSPGRP => Req::SetPgrp,
        TIOCGSID => Req::Sid,
        TIOCOUTQ => Req::OutQueued,
        TCFLSH => Req::Flush,
        TCSBRK | TCSBRKP => Req::Drain,
        TCXONC => Req::FlowControl,
        _ => Req::NotTty,
    }
}

/// TCFLSH's selector, an int: TCIFLUSH and TCIOFLUSH drop typed input,
/// TCOFLUSH has nothing to drop, anything else is EINVAL.
pub fn flush_input(arg: u64) -> Result<bool, i64> {
    match arg as u32 {
        0 | 2 => Ok(true),
        1 => Ok(false),
        _ => Err(errno::EINVAL),
    }
}

/// TCXONC's action, an int: TCOOFF, TCOON, TCIOFF and TCION, each with
/// nothing to do; anything else is EINVAL.
pub fn flow_ok(arg: u64) -> Result<(), i64> {
    match arg as u32 {
        0..=3 => Ok(()),
        _ => Err(errno::EINVAL),
    }
}

/// `struct winsize`'s rows and columns, from the eight bytes at the
/// argument; the pixel sizes after them mean nothing here.
pub fn winsize(raw: &[u8]) -> Option<(u16, u16)> {
    let rows = u16::from_le_bytes(raw.get(0..2)?.try_into().ok()?);
    let cols = u16::from_le_bytes(raw.get(2..4)?.try_into().ok()?);
    Some((rows, cols))
}

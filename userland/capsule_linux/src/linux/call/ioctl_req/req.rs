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

//! The requests a descriptor may be asked.

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

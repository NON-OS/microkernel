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

//! A request number decoded, as the descriptor's kind allows.

use super::codes::{
    FIOCLEX, FIONBIO, FIONCLEX, FIONREAD, TCFLSH, TCGETS, TCSBRK, TCSBRKP, TCSETS, TCSETSF,
    TCSETSW, TCXONC, TIOCGPGRP, TIOCGSID, TIOCGWINSZ, TIOCOUTQ, TIOCSPGRP, TIOCSWINSZ,
};
use super::req::Req;
use crate::linux::guest::Kind;

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

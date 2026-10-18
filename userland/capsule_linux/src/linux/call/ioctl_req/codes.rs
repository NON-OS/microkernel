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

//! The ioctl request numbers, as Linux's asm-generic headers give them.

pub(super) const TCGETS: u64 = 0x5401;
pub(super) const TCSETS: u64 = 0x5402;
pub(super) const TCSETSW: u64 = 0x5403;
pub(super) const TCSETSF: u64 = 0x5404;
pub(super) const TCSBRK: u64 = 0x5409;
pub(super) const TCXONC: u64 = 0x540A;
pub(super) const TCFLSH: u64 = 0x540B;
pub(super) const TIOCGPGRP: u64 = 0x540F;
pub(super) const TIOCSPGRP: u64 = 0x5410;
pub(super) const TIOCOUTQ: u64 = 0x5411;
pub(super) const TIOCGWINSZ: u64 = 0x5413;
pub(super) const TIOCSWINSZ: u64 = 0x5414;
pub(super) const FIONREAD: u64 = 0x541B;
pub(super) const FIONBIO: u64 = 0x5421;
pub(super) const TCSBRKP: u64 = 0x5425;
pub(super) const TIOCGSID: u64 = 0x5429;
pub(super) const FIONCLEX: u64 = 0x5450;
pub(super) const FIOCLEX: u64 = 0x5451;

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

//! Performance and efficiency cores, as each architecture names them (on
//! x86, Intel hybrid parts: `arch::x86_64::cpu::hybrid`). Pure, so the host
//! proofs can hold it.

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CoreKind {
    Unknown = 0,
    Performance = 1,
    Efficiency = 2,
}

impl CoreKind {
    pub(crate) fn from_u8(raw: u8) -> Self {
        match raw {
            1 => CoreKind::Performance,
            2 => CoreKind::Efficiency,
            _ => CoreKind::Unknown,
        }
    }

    pub(crate) fn letter(self) -> &'static [u8] {
        match self {
            CoreKind::Performance => b"P",
            CoreKind::Efficiency => b"E",
            CoreKind::Unknown => b"?",
        }
    }

    /// Which pass of the idle-CPU search offers this core a wake: a woken
    /// task starts on a P-core when one is idle, and on an E-core when none
    /// is. A core of unknown kind is treated as a P-core, as every core on a
    /// part that is not hybrid is.
    pub(crate) fn wake_pass(self) -> usize {
        match self {
            CoreKind::Efficiency => 1,
            CoreKind::Performance | CoreKind::Unknown => 0,
        }
    }
}

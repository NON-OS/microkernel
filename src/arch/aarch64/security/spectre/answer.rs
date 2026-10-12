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

//! What the firmware said, through SMCCC_ARCH_FEATURES, about each
//! workaround on the CPU that asked (DEN0028, the ARCH_WORKAROUND sections).

/// One workaround's answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Answer {
    /// 0: implemented, and this CPU needs it called.
    Call,
    /// 1 (not needed on this CPU) or -2 (NOT_REQUIRED, not needed on any):
    /// the CPU is unaffected, or firmware mitigates it permanently.
    NotRequired,
    /// -1, anything else, or no SMCCC 1.1 to ask: the firmware cannot say.
    Unknown,
}

impl Answer {
    /// The return values as recalled from DEN0028, unverified here.
    pub(crate) const fn from_ret(ret: Option<i64>) -> Self {
        match ret {
            Some(0) => Self::Call,
            Some(1) | Some(-2) => Self::NotRequired,
            _ => Self::Unknown,
        }
    }
}

/// The three answers for one CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Firmware {
    pub(crate) wa1: Answer,
    pub(crate) wa2: Answer,
    pub(crate) wa3: Answer,
}

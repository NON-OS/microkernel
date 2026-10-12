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

//! A transfer descriptor's words (facts::td), built and read. Pure.

use crate::facts::td::*;

/// Who a TD is for: the device's address and endpoint, and whether it runs
/// at low speed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Target {
    pub addr: u8,
    pub ep: u8,
    pub low_speed: bool,
}

/// The control/status word of a TD ready to run: active, three tries
/// before an error, low speed as the target is, and IOC when asked.
pub const fn ctrl_word(t: Target, ioc: bool) -> u32 {
    let mut w = STATUS_ACTIVE | (3 << CTRL_CERR_SHIFT) | ACTLEN_MASK;
    if t.low_speed {
        w |= CTRL_LS;
    }
    if ioc {
        w |= CTRL_IOC;
    }
    w
}

/// The token for `len` bytes (0 for none) with data toggle `toggle`.
pub const fn token(pid: u8, t: Target, toggle: bool, len: usize) -> u32 {
    let max = if len == 0 { LEN_NONE } else { (len as u32 - 1) & LEN_NONE };
    pid as u32
        | (t.addr as u32 & 0x7F) << TOKEN_ADDR_SHIFT
        | (t.ep as u32 & 0xF) << TOKEN_EP_SHIFT
        | if toggle { TOKEN_TOGGLE } else { 0 }
        | max << TOKEN_MAXLEN_SHIFT
}

/// What a TD's control/status word says now.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TdState {
    Active,
    /// Ended without error, with this many bytes moved.
    Done(usize),
    /// Ended with these status bits (facts::td::STATUS_ERRORS).
    Failed(u32),
}

pub const fn state(ctrl: u32) -> TdState {
    if ctrl & STATUS_ACTIVE != 0 {
        return TdState::Active;
    }
    if ctrl & STATUS_ERRORS != 0 {
        return TdState::Failed(ctrl & STATUS_ERRORS);
    }
    // ActLen is the count less one; 0x7FF is none.
    TdState::Done(((ctrl + 1) & ACTLEN_MASK) as usize)
}

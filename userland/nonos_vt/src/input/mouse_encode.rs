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

//! Pointer reports in the SGR form, or the old one-byte form when its
//! coordinates fit.

use alloc::vec::Vec;
use core::fmt::Write;

use super::mouse::{wanted, Button, MouseEvent, MouseKind};
use crate::out::Out;
use crate::term::{Modes, MouseMode};

/// Append the report for `ev`, or return false when the program did not ask
/// for it, or when the old encoding cannot carry its position.
pub fn encode_mouse(ev: &MouseEvent, modes: &Modes, out: &mut Vec<u8>) -> bool {
    if !wanted(ev, modes.mouse) {
        return false;
    }
    let mut cb: u32 = match ev.button {
        Button::Left => 0,
        Button::Middle => 1,
        Button::Right => 2,
        Button::None => 3,
        Button::WheelUp => 64,
        Button::WheelDown => 65,
    };
    if ev.kind == MouseKind::Motion {
        cb += 32;
    }
    if modes.mouse != MouseMode::Press {
        let m = ev.mods;
        cb += 4 * m.shift as u32 + 8 * m.alt as u32 + 16 * m.ctrl as u32;
    }
    let (x, y) = (ev.col + 1, ev.row + 1);
    if modes.mouse_sgr {
        let end = if ev.kind == MouseKind::Release { 'm' } else { 'M' };
        let _ = write!(Out(out), "\x1b[<{cb};{x};{y}{end}");
        return true;
    }
    // The old form has no release per button and one byte per coordinate.
    if ev.kind == MouseKind::Release {
        cb = (cb & !3) | 3;
    }
    if x > 223 || y > 223 {
        return false;
    }
    out.extend_from_slice(b"\x1b[M");
    out.extend_from_slice(&[(cb + 32) as u8, (x + 32) as u8, (y + 32) as u8]);
    true
}

/// `CSI I` on focus, `CSI O` on losing it, when the program asked.
pub fn encode_focus(focused: bool, modes: &Modes, out: &mut Vec<u8>) -> bool {
    if !modes.focus {
        return false;
    }
    out.extend_from_slice(if focused { b"\x1b[I" } else { b"\x1b[O" });
    true
}

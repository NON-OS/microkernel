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

//! The AMD bank: arming writes the trigger, level and enables and never a
//! status bit; a pending pin is cleared alone and the bank's end of
//! interrupt follows; pins it cannot place refused.

use super::amd_model::bank;
use nonos_gpio::amd_bits::*;
use nonos_gpio::{Controller, GpioError, Polarity, Trigger};

const EDGE_LOW: Trigger = Trigger { edge: true, polarity: Polarity::Low };
const LEVEL_HIGH: Trigger = Trigger { edge: false, polarity: Polarity::High };

#[test]
fn arming_each_trigger() {
    let mut b = bank();
    b.window.regs.insert(5 * 4, WAKE_STS | OUTPUT_ENABLE);
    b.arm(5, EDGE_LOW).unwrap();
    let v = b.window.pin(5);
    assert_eq!(v & (LEVEL_TRIG | ACTIVE_LEVEL_MASK), 1 << ACTIVE_LEVEL_SHIFT);
    assert_eq!(
        v & (INTERRUPT_ENABLE | INTERRUPT_DELIVER | OUTPUT_ENABLE | WAKE_STS),
        INTERRUPT_ENABLE | INTERRUPT_DELIVER | OUTPUT_ENABLE | WAKE_STS,
        "wake status kept, not cleared"
    );
    b.arm(6, LEVEL_HIGH).unwrap();
    assert_eq!(b.window.pin(6) & (LEVEL_TRIG | ACTIVE_LEVEL_MASK), LEVEL_TRIG);
    b.arm(7, Trigger { edge: true, polarity: Polarity::Both }).unwrap();
    assert_eq!(b.window.pin(7) & ACTIVE_LEVEL_MASK, 2 << ACTIVE_LEVEL_SHIFT);
    assert_eq!(b.arm(63, EDGE_LOW), Err(GpioError::Unplaced), "the master register's dword");
    assert_eq!(b.arm(256, EDGE_LOW), Err(GpioError::Unplaced), "past the window");
}

#[test]
fn pending_cleared_alone_then_end_of_interrupt() {
    let mut b = bank();
    for p in [3, 4, 9] {
        b.arm(p, EDGE_LOW).unwrap();
    }
    b.window.raise(3);
    b.window.raise(9);
    b.window.regs.entry(9 * 4).and_modify(|v| *v |= WAKE_STS);
    assert_eq!(b.take_pending(&[3, 4, 9]), vec![3, 9]);
    assert_eq!(b.window.pin(3) & INTERRUPT_STS, 0);
    assert_eq!(b.window.pin(9) & (INTERRUPT_STS | WAKE_STS), WAKE_STS, "wake status left set");
    assert_eq!(b.window.eois, 1);
    assert!(b.take_pending(&[3, 4, 9]).is_empty());
}

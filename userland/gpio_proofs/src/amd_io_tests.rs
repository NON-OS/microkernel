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

//! The AMD bank's masks, levels and outputs: masking holds back delivery
//! and unmasking restores it; an output is driven only where the pin's
//! output enable is set.

use super::amd_model::bank;
use nonos_gpio::amd_bits::*;
use nonos_gpio::{Controller, GpioError, Polarity, Trigger};

const EDGE_LOW: Trigger = Trigger { edge: true, polarity: Polarity::Low };

#[test]
fn masking_levels_and_outputs() {
    let mut b = bank();
    b.arm(2, EDGE_LOW).unwrap();
    b.mask(2, true).unwrap();
    assert_eq!(b.window.pin(2) & INTERRUPT_DELIVER, 0);
    b.mask(2, false).unwrap();
    assert_ne!(b.window.pin(2) & INTERRUPT_DELIVER, 0);
    b.window.regs.insert(8 * 4, PIN_STS);
    assert_eq!(b.level(8), Ok(true));
    assert_eq!(b.drive(8, true), Err(GpioError::NotOutput));
    b.window.regs.insert(8 * 4, OUTPUT_ENABLE);
    b.drive(8, true).unwrap();
    assert_ne!(b.window.pin(8) & OUTPUT_VALUE, 0);
    b.drive(8, false).unwrap();
    assert_eq!(b.window.pin(8) & OUTPUT_VALUE, 0);
}

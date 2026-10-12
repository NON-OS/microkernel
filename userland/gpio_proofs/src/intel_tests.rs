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

//! Intel communities: arming writes the pad's event configuration and
//! inversion and its group enable, leaving mode and routing as firmware
//! set them; a native-function pad, a disabled input and an unplaced pin
//! are refused; a pending pin is cleared alone, a masked one is not seen.

use super::intel_model::gpio;
use super::intel_pins::{dw0, raise, set_dw0};
use nonos_gpio::intel_bits::*;
use nonos_gpio::{Controller, GpioError, Polarity, Trigger};

const ROUTE_SCI: u32 = 1 << 19;

#[test]
fn arming_and_refusals() {
    let mut g = gpio();
    set_dw0(&mut g, 33, ROUTE_SCI);
    g.arm(33, Trigger { edge: true, polarity: Polarity::Low }).unwrap();
    assert_eq!(dw0(&mut g, 33) & (RX_EVCFG_MASK | RX_INVERT), 1 << RX_EVCFG_SHIFT | RX_INVERT);
    assert_ne!(dw0(&mut g, 33) & ROUTE_SCI, 0, "routing left as firmware set it");
    g.arm(101, Trigger { edge: true, polarity: Polarity::Both }).unwrap();
    assert_eq!(dw0(&mut g, 101) & (RX_EVCFG_MASK | RX_INVERT), 3 << RX_EVCFG_SHIFT);
    g.arm(2, Trigger { edge: false, polarity: Polarity::High }).unwrap();
    assert_eq!(dw0(&mut g, 2) & RX_EVCFG_MASK, 0);
    set_dw0(&mut g, 4, 1 << 10);
    assert_eq!(g.arm(4, Trigger { edge: true, polarity: Polarity::High }), Err(GpioError::Trigger));
    set_dw0(&mut g, 5, RX_DISABLE);
    assert_eq!(g.arm(5, Trigger { edge: true, polarity: Polarity::High }), Err(GpioError::Trigger));
    assert_eq!(
        g.arm(80, Trigger { edge: true, polarity: Polarity::High }),
        Err(GpioError::Unplaced)
    );
}

#[test]
fn pending_masked_levels_and_outputs() {
    let mut g = gpio();
    let edge = Trigger { edge: true, polarity: Polarity::High };
    for p in [1, 3, 33, 100] {
        g.arm(p, edge).unwrap();
    }
    for p in [1, 3, 100] {
        raise(&mut g, p);
    }
    g.mask(33, true).unwrap();
    raise(&mut g, 33);
    assert_eq!(g.take_pending(&[1, 3, 33, 100]), vec![1, 3, 100], "pin 33 is masked");
    assert!(g.take_pending(&[1, 3, 33, 100]).is_empty(), "each cleared");
    set_dw0(&mut g, 7, RX_STATE | TX_DISABLE);
    assert_eq!(g.level(7), Ok(true));
    assert_eq!(g.drive(7, true), Err(GpioError::NotOutput));
    set_dw0(&mut g, 7, 0);
    g.drive(7, true).unwrap();
    assert_eq!(dw0(&mut g, 7) & TX_STATE, TX_STATE);
}

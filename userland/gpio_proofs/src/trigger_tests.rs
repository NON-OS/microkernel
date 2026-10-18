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

//! A GpioInt's interrupt flags into a trigger: every polarity on an edge,
//! high and low on a level, and the combinations no pin can honour refused.

use nonos_gpio::{Polarity, Trigger};

#[test]
fn every_flag_combination() {
    let t = |edge, polarity| Some(Trigger { edge, polarity });
    assert_eq!(Trigger::from_flags(0b001), t(true, Polarity::High));
    assert_eq!(Trigger::from_flags(0b011), t(true, Polarity::Low));
    assert_eq!(Trigger::from_flags(0b101), t(true, Polarity::Both));
    assert_eq!(Trigger::from_flags(0b000), t(false, Polarity::High));
    assert_eq!(Trigger::from_flags(0b010), t(false, Polarity::Low));
    assert_eq!(Trigger::from_flags(0b100), None, "a level on both polarities");
    assert_eq!(Trigger::from_flags(0b111), None, "polarity 3");
    assert_eq!(
        Trigger::from_flags(0b11_001),
        t(true, Polarity::High),
        "shared and wake bits ignored"
    );
}

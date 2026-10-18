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

//! The backlight PWM as a share of firmware's period.

use crate::engine::backlight::{get, set, BLC_PWM_PCH_CTL1, BLC_PWM_PCH_CTL2, PWM_ENABLE};
use crate::engine::error::Error;
use crate::model::Gen9;

#[test]
fn a_level_sets_its_share_of_the_period_and_enables_the_pwm() {
    let mut m = Gen9::new(1);
    m.set(BLC_PWM_PCH_CTL2, 1000 << 16);
    set(&mut m, 30, 100).unwrap();
    assert_eq!(m.get(BLC_PWM_PCH_CTL2), (1000 << 16) | 300, "period kept");
    assert_eq!(m.get(BLC_PWM_PCH_CTL1) & PWM_ENABLE, PWM_ENABLE);
    assert_eq!(get(&mut m, 100), 30);
    set(&mut m, 255, 255).unwrap();
    assert_eq!(get(&mut m, 255), 255);
}

#[test]
fn no_period_or_a_level_past_max_is_refused_unwritten() {
    let mut m = Gen9::new(1);
    assert_eq!(set(&mut m, 1, 2), Err(Error::Argument));
    m.set(BLC_PWM_PCH_CTL2, 1000 << 16);
    assert_eq!(set(&mut m, 3, 2), Err(Error::Argument));
    assert_eq!(set(&mut m, 0, 0), Err(Error::Argument));
    assert!(m.writes.is_empty());
    assert_eq!(get(&mut Gen9::new(1), 100), 0);
}

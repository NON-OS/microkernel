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

//! The slot, looked at every 500 ms. A card that arrives is brought up and
//! read; one that fails is powered down again and left until it is
//! reinserted, so a bad card does not fill the log; one that leaves powers
//! the slot down.

use crate::card::{bring_up, power_off, present};
use crate::clock::sleep_ms;
use crate::report;
use crate::setup::Driver;

const LOOK_MS: u64 = 500;

pub fn run(drv: &mut Driver) -> ! {
    let mut seated = false;
    loop {
        let here = present(drv);
        if here && !seated {
            let card = bring_up(drv);
            report::card(drv, card);
            if card.is_err() {
                let _ = power_off(drv);
            }
        }
        if !here && seated {
            report::removed(power_off(drv));
        }
        seated = here;
        sleep_ms(LOOK_MS);
    }
}

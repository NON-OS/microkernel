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

//! What the server does after each request: an OK answer counts toward
//! the device's span of service (`streak`); a stall is reset by name and
//! the device brought up again, until a run of resets has gone on too
//! long, when the device is given up instead.

use super::recover::{give_up, recover};
use super::streak::Streak;
use crate::clock;
use crate::setup::Driver;

pub fn settle(driver: &mut Driver, streak: &mut Streak) {
    if driver.queue.take_served() {
        streak.served(clock::uptime_ms());
    }
    let Some(why) = driver.queue.stalled() else {
        return;
    };
    if !streak.may_reset() {
        give_up(driver, why);
    }
    recover(driver, why);
    streak.reset(clock::uptime_ms());
}

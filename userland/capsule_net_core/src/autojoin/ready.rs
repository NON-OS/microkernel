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

//! Whether a pass may try at all: this boot keeps state, the Wi-Fi switch is
//! not off, and a driver that can join is up with no link yet. A driver that
//! cannot join ends the passes before the saved list is opened, so no
//! passphrase is unsealed for it.

use nonos_policy_proto::Field;
use nonos_wifi_client::{find, Driver, DriverStage};

use super::machine::{Step, CALL_MS};

/// The driver to join with, or the step the pass ends on.
pub(super) fn ready() -> Result<Driver, Step> {
    let Some(policy) = nonos_policy_client::lookup() else { return Err(Step::NotYet) };
    let get = |f| nonos_policy_client::get_bool_within(policy, f, CALL_MS);
    if get(Field::Persistent) != Some(true) || get(Field::WifiRadio) == Some(false) {
        return Err(Step::NotYet);
    }
    let Some(driver) = find() else { return Err(Step::NotYet) };
    if !driver.joins() {
        return Err(Step::Done);
    }
    match driver.stage_within(CALL_MS) {
        Some(DriverStage::Ready) => {}
        Some(DriverStage::NoAirPath) => return Err(Step::Done),
        _ => return Err(Step::NotYet),
    }
    if driver.link_within(CALL_MS).is_some_and(|l| l.associated) {
        return Err(Step::Done);
    }
    Ok(driver)
}

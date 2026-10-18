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

//! The join a pass hands the driver, and what its answer means.

use nonos_wifi_client::{join_text, Driver, SavedList};

use super::machine::{step_for, Step, CALL_MS};
use super::tick::say;

/// Hand the driver saved network `i`: started when it does not answer within
/// `CALL_MS` (the join runs on in the driver), or answered. A rejoin after a
/// loss is flagged, so the driver refuses it if the person left since.
pub(super) fn join(driver: &Driver, list: &SavedList, i: usize, rejoin: bool) -> Step {
    let index = i as u8;
    let (ssid, pass, flags) = (list.ssid(i), list.passphrase(i), list.join_flags(i));
    let answer = if rejoin {
        driver.rejoin_within(ssid, pass, flags, CALL_MS)
    } else {
        driver.connect_within(ssid, pass, flags, CALL_MS)
    };
    match answer {
        None => {
            say(b"[NET-CORE] joining a saved Wi-Fi network\n");
            Step::Started(index)
        }
        Some(r) => {
            if r.code == 0 {
                say(b"[NET-CORE] joined a saved Wi-Fi network\n");
            } else {
                say(b"[NET-CORE] saved Wi-Fi network not joined: ");
                say(join_text(r.code).as_bytes());
                say(b"\n");
            }
            step_for(index, r.code)
        }
    }
}

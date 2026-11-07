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

//! What a tick does for the page that is open. The Wi-Fi and Network pages
//! re-poll net_core, so a lease that binds a few seconds after connecting
//! shows its address without another scan. The Bluetooth page asks
//! driver.bt_usb0 again, which sends nothing unprompted (decision 7 of
//! letter E-25), so a question pairing asks is seen only by asking.

use crate::bluetooth::{panel::poll_ms, poll};
use crate::wifi::{DataPath, DriverStage, NetStatus};
use nonos_wifi_client::{Driver, Link, StopWhy};

use super::section::Section;
use super::state::wifi::status::refresh_wifi_status;
use super::state::State;
use super::tick_view::reread;

/// Whether the page changed and wants painting: the Wi-Fi, Network and
/// Security pages when what they show came back different (tick_view.rs).
/// The Bluetooth page is painted each poll, since its pairing panel counts
/// its window down.
pub fn page_tick(state: &mut State) -> bool {
    match state.section {
        Section::Wifi | Section::Network => reread(state, wifi_shown, refresh_wifi_status),
        Section::Bluetooth => {
            poll(&mut state.bt, false);
            true
        }
        // Until one is set: the shell's prompt may set it while this page
        // is open. Once set it stays set.
        Section::Security if state.lock_set != Some(true) => {
            reread(state, |s| s.lock_set, |s| s.lock_set = super::actions::lock_set())
        }
        _ => false,
    }
}

/// What the Wi-Fi and Network pages show of what a tick re-reads: the
/// driver, its stage and why it stopped, the link, whether a network can be
/// kept, the data path, the lease and the station. Not when the lease was
/// last asked, which changes every second and is not drawn.
type WifiShown = (
    Option<Driver>,
    Option<DriverStage>,
    Option<StopWhy>,
    Option<Link>,
    (bool, bool),
    Option<DataPath>,
    NetStatus,
    Option<u32>,
);

fn wifi_shown(s: &State) -> WifiShown {
    let (w, d) = (&s.wifi, &s.wifi_diag);
    (w.driver, d.stage, d.stop, w.link, (w.keeps, w.remember), d.datapath, d.net, d.station)
}

pub fn page_tick_ms(state: &State) -> i64 {
    match state.section {
        Section::Bluetooth => poll_ms(&state.bt),
        _ => 1000,
    }
}

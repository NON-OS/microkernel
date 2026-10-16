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

//! The worker's own run: the one call to the driver made and waited on, a
//! joined network remembered while the passphrase is still held, and the
//! answer left where a tick picks it up.

use super::super::state::WIFI_NET_MAX;
use super::worker::ANSWER;
use super::worker_msg::{Answer, Ask};
use crate::wifi::ScanNetwork;
use alloc::boxed::Box;
use nonos_wifi_client::remember;

pub(super) fn work(arg: usize) {
    // SAFETY: `arg` is the box `send` leaked for this worker, and the spawn
    // succeeded, so the worker owns it from here; dropping it wipes the
    // passphrase.
    let ask = unsafe { Box::from_raw(arg as *mut Ask) };
    let answer = perform(&ask);
    drop(ask);
    ANSWER.put(answer);
}

/// Ask the driver, on whichever thread calls this.
pub(super) fn perform(ask: &Ask) -> Answer {
    match ask {
        Ask::Scan(driver) => {
            let mut nets = [ScanNetwork::EMPTY; WIFI_NET_MAX];
            let (count, outcome, stats) = driver.scan(&mut nets);
            Answer::Scanned { nets, count, outcome, stats }
        }
        Ask::Join { driver, net, pass, remember: keep } => {
            let result = driver.connect(net.ssid(), pass.as_slice());
            let kept =
                (result.code == 0 && *keep).then(|| match remember(net.ssid(), pass.as_slice()) {
                    Ok(()) => "Remembered, sealed with this machine's TPM key",
                    Err(e) => e.text(),
                });
            Answer::Joined { result, kept }
        }
    }
}

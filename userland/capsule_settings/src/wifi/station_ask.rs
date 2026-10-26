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
//! The station source asked of driver.netvsc0, the one driver that reports
//! it (station.rs). It is decided before the link first comes up and does
//! not change while the system runs, so a page asks until it is answered.

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use super::station::{station_of, stats_request, REPLY_LEN};

const DRIVER: &[u8] = b"driver.netvsc0";
const TIMEOUT_MS: u64 = 250;

/// The source word, or None when there is no such driver or no answer.
pub fn station_source(id: u32) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(DRIVER.as_ptr(), DRIVER.len(), &mut port, &mut pid) < 0 || port == 0 {
        return None;
    }
    let tx = stats_request(id);
    let mut rx = [0u8; REPLY_LEN];
    let (tl, rl) = (tx.len(), rx.len());
    let got = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tl, rx.as_mut_ptr(), rl, TIMEOUT_MS);
    let end = usize::try_from(got).ok().filter(|&g| g <= rx.len())?;
    station_of(&rx[..end], id)
}

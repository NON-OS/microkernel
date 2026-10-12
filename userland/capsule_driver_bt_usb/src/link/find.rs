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

//! Looking for a HID device with a passive scan: the controller only
//! listens, so nothing of ours goes on the air while looking. A bonded
//! peer is taken the moment it is heard, directed or not; a new one waits
//! a little while bonds are kept, so a bonded keyboard waking beside a
//! stranger's is not passed over for it.

use super::env::Env;
use super::listen::listen;
use nonos_bt_hci::host::DEFAULT_SCAN;
use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_wire::addr::{AddrType, BdAddr};

/// One look lasts this long and the scan then stops, so a capsule with no
/// device in range does not keep the radio listening without end.
pub const SCAN_MS: u64 = 10_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Peer {
    pub kind: AddrType,
    pub addr: BdAddr,
}

/// The peer to connect to, if one is heard within SCAN_MS. The scan is
/// stopped whatever happened, before any error is returned, so a failed
/// look never leaves the controller scanning.
pub fn find_hid<T: Transport, E: Env>(
    host: &mut Host<T>,
    env: &mut E,
) -> Result<Option<Peer>, HciError> {
    host.scan_passive(DEFAULT_SCAN)?;
    let heard = listen(host, env);
    let stopped = host.scan_stop();
    let heard = heard?;
    stopped?;
    Ok(heard)
}

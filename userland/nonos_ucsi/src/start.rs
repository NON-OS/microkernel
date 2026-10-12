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

//! Bringing the PPM up from nothing: reset, notifications on, how many
//! connectors, and each connector's capability and status. The same walk
//! recovers a PPM that stuck.

use super::capability::{connector_capability, connectors, ConnectorCapability};
use super::command::NOTIFY;
use super::command::{capability, connector_capability as cap_of, connector_status, notifications};
use super::exchange::{command, reset, Fault};
use super::ppm::Ppm;
use super::status::{status, Status};
use alloc::vec::Vec;

/// Connectors kept: the wire serves at most this many, and a PPM that
/// claims more is believed only this far.
pub const MAX_CONNECTORS: u8 = 16;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Port {
    pub capability: Option<ConnectorCapability>,
    pub status: Option<Status>,
}

/// One connector's status, acknowledging its change when `change`.
pub fn read_port(
    ppm: &mut impl Ppm,
    pending: &mut u128,
    n: u8,
    change: bool,
) -> Result<Option<Status>, Fault> {
    match command(ppm, pending, connector_status(n), change) {
        Ok(b) => Ok(status(&b)),
        Err(Fault::Failed | Fault::NotSupported) => Ok(None),
        Err(e) => Err(e),
    }
}

pub fn start(ppm: &mut impl Ppm, pending: &mut u128) -> Result<Vec<Port>, Fault> {
    reset(ppm, pending)?;
    command(ppm, pending, notifications(NOTIFY), false)?;
    let n = connectors(&command(ppm, pending, capability(), false)?).unwrap_or(0);
    let mut ports = Vec::new();
    for c in 1..=n.min(MAX_CONNECTORS) {
        let capability = match command(ppm, pending, cap_of(c), false) {
            Ok(b) => connector_capability(&b),
            Err(Fault::Failed | Fault::NotSupported) => None,
            Err(e) => return Err(e),
        };
        let status = read_port(ppm, pending, c, true)?;
        ports.push(Port { capability, status });
    }
    // What the walk read is current; changes it acknowledged are done.
    *pending = 0;
    Ok(ports)
}

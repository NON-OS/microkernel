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

//! The addresses a join transmits from: a scan's own while it probes for a
//! hidden network the person named, then the network's own from the first
//! authentication frame on. A chip that cannot take one ends the join,
//! rather than show the network an address another network has seen.

use super::joined::JoinError;
use super::request::ConnectRequest;
use crate::address::{security_of, Addresses};
use crate::softmac::radio::{Env, Radio};

/// The network's address for `req`.
pub(super) fn network_address(a: &Addresses, req: &ConnectRequest) -> [u8; 6] {
    a.join(req.ssid, security_of(req.wpa3_only, req.pass))
}

/// Put a fresh scan address on the radio, for the probes of a hidden join.
pub(super) fn scan_from<R: Radio, E: Env>(
    r: &mut R,
    env: &mut E,
    a: &mut Addresses,
    net: [u8; 6],
) -> Result<(), JoinError> {
    let scan = a.scan(net, |b| env.random(b)).ok_or(JoinError::NoEntropy)?;
    r.set_mac(scan).then_some(()).ok_or(JoinError::NoAddress)
}

/// Put the network's own address on the radio.
pub(super) fn join_from<R: Radio>(r: &mut R, net: [u8; 6]) -> Result<(), JoinError> {
    r.set_mac(net).then_some(()).ok_or(JoinError::NoAddress)
}

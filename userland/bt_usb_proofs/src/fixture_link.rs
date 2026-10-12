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

//! A link from scan to its end, as the capsule's `run::links` runs one.

use crate::env::TestEnv;
use crate::fixture::Stack;
use crate::link::session::Session;
use crate::link::us::Us;
use crate::link::{begin, find_hid, serve, End};
use nonos_bt_hci::host::{Connect, DEFAULT_LINK, DEFAULT_SCAN};
use nonos_bt_hci::HciError;
use nonos_bt_wire::addr::{AddrType, BdAddr};

/// Find, connect, pair and serve from the public address; how the link
/// ended and its session.
pub fn link(
    host: &mut Stack,
    ours: &BdAddr,
    env: &mut TestEnv,
) -> Result<(End, Session), HciError> {
    link_as(host, &Us { kind: AddrType::Public, addr: *ours, identity: None }, env)
}

/// As `link`, as `us`; a bonded peer is encrypted with its kept key.
pub fn link_as(host: &mut Stack, us: &Us, env: &mut TestEnv) -> Result<(End, Session), HciError> {
    let peer = find_hid(host, env)?.expect("no HID device heard");
    let c =
        Connect { scan: DEFAULT_SCAN, peer_type: peer.kind, peer: peer.addr, link: DEFAULT_LINK };
    let link = host.connect(&c)?;
    let mut s = begin(host, &link, us, env)?.expect("pairing did not start");
    let end = serve(host, &mut s, env)?;
    Ok((end, s))
}

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
//! Cover packets, built to be the size and shape of a real one: the same
//! fragment length, the same acknowledgement inside, a fresh route. A loop
//! cover is addressed to this client through its own gateway, comes back,
//! and is dropped on arrival, since no reply block key opens it. A drop
//! cover is addressed to no client behind a gateway drawn from the directory,
//! which discards it.

extern crate alloc;

use alloc::vec::Vec;

use crate::ack::FRAG_ID_BYTES;
use crate::crypto::random::fill_random;
use crate::message::FRAGMENT_PER_PACKET;
use crate::mixnet::cover_clock::Tick;
use crate::mixnet::{encode_fragment, route_home, Addressed};
use crate::state::{ack_key, client_identity, TABLE};
use crate::topology;

/// A cover packet for `tick`, or None when the client cannot route yet.
pub fn cover(tick: Tick) -> Option<Vec<u8>> {
    let identity = client_identity()?;
    let key = ack_key()?;
    let ours = TABLE.lock().gateway()?.identity;
    let mut seed = [0u8; 32];
    fill_random(&mut seed).ok()?;
    let home = route_home(&seed, &ours)?;
    let mut random = [0u8; 32 + 32 + FRAG_ID_BYTES];
    fill_random(&mut random).ok()?;
    let mut encryption = [0u8; 32];
    encryption.copy_from_slice(&random[..32]);
    let mut frag_id = [0u8; FRAG_ID_BYTES];
    frag_id.copy_from_slice(&random[64..]);
    let (destination, gateway) = match tick {
        Tick::Loop => (identity.public, ours),
        Tick::Send => {
            fill_random(&mut seed).ok()?;
            let far = topology::route(&seed).ok()?;
            let mut nobody = [0u8; 32];
            nobody.copy_from_slice(&random[32..64]);
            (nobody, far[far.len() - 1].identity)
        }
    };
    let mut fragment = alloc::vec![0u8; FRAGMENT_PER_PACKET];
    fill_random(&mut fragment).ok()?;
    let addressed = Addressed {
        destination: &destination,
        destination_encryption: &encryption,
        destination_gateway: &gateway,
        our_identity: &identity.public,
        ack_key: &key,
        home: &home,
        sender_tag: &[0u8; 16],
        reply_surbs: &[],
    };
    encode_fragment(&addressed, frag_id, &fragment).map(|(packet, _)| packet)
}

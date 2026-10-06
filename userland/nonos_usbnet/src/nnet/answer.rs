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

//! Answering one NNET request. Pure over the `Nic`, so the proofs drive it
//! with a scripted device; the runner only receives and sends.

use super::encode::{reply, status};
use super::frames::{recv, send};
use super::stats::Stats;
use super::wire::OP_TX_PACKET;
use super::wire::{Request, DATA_AT, E_AGAIN, E_INVAL, E_MSGSIZE};
use super::wire::{OP_HEALTHCHECK, OP_LINK_STATUS, OP_MAC_ADDRESS, OP_RX_PACKET, OP_STATS};
use crate::nic::{Nic, ETH_FRAME_MAX, ETH_HEADER};

/// The reply to `req` in `tx`, its length returned. Before a device is
/// bound (`nic` is `None`) the link reads down and frames are `E_AGAIN`.
pub fn answer<N: Nic>(
    nic: Option<&mut N>,
    stats: &mut Stats,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    match (req.op, nic) {
        (OP_HEALTHCHECK, _) => status(tx, req, 0),
        (OP_LINK_STATUS, nic) => {
            tx[DATA_AT] = nic.is_some_and(|n| n.link_up()) as u8;
            reply(tx, req, 0, 1)
        }
        (OP_STATS, _) => {
            let n = stats.encode(&mut tx[DATA_AT..]);
            reply(tx, req, 0, n)
        }
        (OP_MAC_ADDRESS, Some(nic)) => {
            tx[DATA_AT..DATA_AT + 6].copy_from_slice(&nic.mac());
            reply(tx, req, 0, 6)
        }
        (OP_TX_PACKET, Some(_)) if req.payload_len as usize != body.len() => {
            status(tx, req, E_MSGSIZE)
        }
        (OP_TX_PACKET, Some(_)) if !(ETH_HEADER..=ETH_FRAME_MAX).contains(&body.len()) => {
            status(tx, req, E_INVAL)
        }
        (OP_TX_PACKET, Some(nic)) => send(nic, stats, req, body, tx),
        (OP_RX_PACKET, Some(nic)) => recv(nic, stats, req, tx),
        (OP_MAC_ADDRESS | OP_TX_PACKET | OP_RX_PACKET, None) => status(tx, req, E_AGAIN),
        _ => status(tx, req, E_INVAL),
    }
}

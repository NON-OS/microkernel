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

//! A frame down to the device and one up from it.

use super::encode::{reply, status};
use super::stats::Stats;
use super::wire::{Request, DATA_AT, E_AGAIN, RX_PREFIX_LEN};
use crate::nic::{Nic, ETH_FRAME_MAX};

pub(super) fn send<N: Nic>(
    nic: &mut N,
    stats: &mut Stats,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> usize {
    let sent = nic.send(body);
    stats.tx(sent.map(|()| body.len()));
    status(tx, req, sent.err().unwrap_or(0))
}

/// The frame as `len_le32, bytes`, or `E_AGAIN` when none has come.
pub(super) fn recv<N: Nic>(nic: &mut N, stats: &mut Stats, req: &Request, tx: &mut [u8]) -> usize {
    let at = DATA_AT + RX_PREFIX_LEN;
    let got = nic.recv(&mut tx[at..at + ETH_FRAME_MAX]);
    stats.rx(got);
    match got {
        Ok(Some(n)) => {
            tx[DATA_AT..at].copy_from_slice(&(n as u32).to_le_bytes());
            reply(tx, req, 0, RX_PREFIX_LEN + n)
        }
        Ok(None) => status(tx, req, E_AGAIN),
        Err(e) => status(tx, req, e),
    }
}

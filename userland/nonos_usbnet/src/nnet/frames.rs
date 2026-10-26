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
use nonos_mac::resend::{asked, serve, Kept};

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

/// The frame as `len_le32, bytes`, or `E_AGAIN` when none has come. A
/// request numbered as the last one gets the same frame again, its reply
/// having come too late (nonos_mac::resend); only a fresh frame is counted.
pub(super) fn recv<N: Nic>(
    nic: &mut N,
    stats: &mut Stats,
    kept: &mut Kept<ETH_FRAME_MAX>,
    (req, body): (&Request, &[u8]),
    tx: &mut [u8],
) -> usize {
    let at = DATA_AT + RX_PREFIX_LEN;
    let got = serve(kept, asked(body), |buf| {
        let got = nic.recv(buf);
        stats.rx(got);
        got
    });
    match got {
        Ok(Some(frame)) => {
            let n = frame.len();
            tx[at..at + n].copy_from_slice(frame);
            tx[DATA_AT..at].copy_from_slice(&(n as u32).to_le_bytes());
            reply(tx, req, 0, RX_PREFIX_LEN + n)
        }
        Ok(None) => status(tx, req, E_AGAIN),
        Err(e) => status(tx, req, e),
    }
}

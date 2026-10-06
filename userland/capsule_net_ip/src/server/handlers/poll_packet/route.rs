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

use crate::ingress::{from_frame, Inbound, IngressError};
use crate::l2_client::{poll_frame, RxError};
use crate::protocol::E_L2_FAULT;
use crate::state::{push, Packet};

pub enum PollResult {
    KeepPolling,
    Empty,
    Fault(u16),
}

pub fn poll_and_route(l2: u32) -> PollResult {
    let frame = match poll_frame(l2) {
        Ok(f) => f,
        Err(RxError::Empty) => return PollResult::Empty,
        Err(_) => return PollResult::Fault(E_L2_FAULT),
    };
    match from_frame(&frame).map(Packet::from) {
        Ok(p) => {
            let _ = push(p);
            PollResult::KeepPolling
        }
        /*
         * A frame that is not ours, or not well formed, is dropped and the
         * poll goes on. A malformed one used to end the poll as E_BAD_PACKET,
         * as if the link had failed, holding back what was queued behind it.
         */
        Err(IngressError::NotIpv4)
        | Err(IngressError::NotForUs)
        | Err(IngressError::Absorbed)
        | Err(IngressError::BadFrame)
        | Err(IngressError::BadIp) => PollResult::KeepPolling,
    }
}

impl From<Inbound> for Packet {
    fn from(p: Inbound) -> Self {
        Self { src: p.src, dst: p.dst, protocol: p.protocol, payload: p.payload }
    }
}

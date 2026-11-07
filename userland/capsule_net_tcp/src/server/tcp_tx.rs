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

use alloc::vec;

use crate::ip_client::send_segment;
use crate::state::ip_port;
use crate::tcp::syn_profile::{syn_options, NORMALISED};
use crate::tcp::{build, BuildRequest, Endpoint4, Tcb, FLAG_ACK, FLAG_RST, FLAG_SYN, MSS};

pub fn send(tcb: Tcb, flags: u8, payload: &[u8]) -> Result<(), &'static str> {
    // Options ride on the SYN and SYN-ACK only (syn_profile.rs).
    let (area, len) = if flags & FLAG_SYN != 0 { syn_options(NORMALISED, MSS as u16) } else { ([0; 4], 0) };
    let mut seg = vec![0u8; 20 + len + payload.len()];
    let req = BuildRequest {
        src: tcb.local.ip,
        dst: tcb.remote.ip,
        src_port: tcb.local.port,
        dst_port: tcb.remote.port,
        seq: tcb.send.nxt,
        ack: tcb.recv.nxt,
        flags,
        window: tcb.recv.wnd,
        options: &area[..len],
        payload,
    };
    let n = build(&req, &mut seg).map_err(|_| "tcp build failed")?;
    send_segment(ip_port(), tcb.remote.ip, &seg[..n]).map_err(|_| "tcp send failed")
}

pub fn send_rst(local: Endpoint4, remote: Endpoint4, seq: u32, ack: u32) -> Result<(), &'static str> {
    let mut seg = vec![0u8; 20];
    let flags = if ack == 0 { FLAG_RST } else { FLAG_RST | FLAG_ACK };
    let req = BuildRequest {
        src: local.ip,
        dst: remote.ip,
        src_port: local.port,
        dst_port: remote.port,
        seq,
        ack,
        flags,
        window: 0,
        options: &[],
        payload: &[],
    };
    let n = build(&req, &mut seg).map_err(|_| "tcp build failed")?;
    send_segment(ip_port(), remote.ip, &seg[..n]).map_err(|_| "tcp send failed")
}

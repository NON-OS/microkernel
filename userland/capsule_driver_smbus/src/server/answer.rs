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

//! One request answered. Generic over the host's ports, for the proofs.

use crate::errno::{E_INVAL, E_OK};
use crate::ports::Ports;
use crate::wire::{decode_request, encode_response_header, refused, write_status, Request};
use crate::wire::{HDR_LEN, INFO_LEN, OP_HEALTHCHECK, OP_INFO, OP_TRANSFER, STATUS_LEN};
use crate::xfer::{parse, transfer, BLOCK_MAX};

/// What OP_INFO reports.
#[derive(Clone, Copy)]
pub struct Info {
    pub vendor: u16,
    pub device: u16,
    pub io_ports: u16,
}

/// Answers `msg` into `tx` and returns the reply's length. A frame that does
/// not decode is answered too: its caller waits in its call for one.
pub fn answer<P: Ports>(host: &mut P, info: &Info, msg: &[u8], tx: &mut [u8]) -> usize {
    let Some(req) = decode_request(msg) else {
        return reply(tx, &refused(msg), E_INVAL, &[]);
    };
    let Some(payload) = msg.get(HDR_LEN..HDR_LEN.saturating_add(req.payload_len as usize)) else {
        return reply(tx, &req, E_INVAL, &[]);
    };
    match req.op {
        OP_HEALTHCHECK if payload.is_empty() => reply(tx, &req, E_OK, &[]),
        OP_INFO if payload.is_empty() => {
            let mut b = [0u8; INFO_LEN];
            b[0..2].copy_from_slice(&info.vendor.to_le_bytes());
            b[2..4].copy_from_slice(&info.device.to_le_bytes());
            b[4..6].copy_from_slice(&info.io_ports.to_le_bytes());
            reply(tx, &req, E_OK, &b)
        }
        OP_TRANSFER => {
            let Some(x) = parse(payload) else {
                return reply(tx, &req, E_INVAL, &[]);
            };
            let mut data = [0u8; BLOCK_MAX];
            match transfer(host, &x, &mut data) {
                Ok(n) => {
                    let mut out = [0u8; BLOCK_MAX + 1];
                    out[0] = n as u8;
                    out[1..=n].copy_from_slice(&data[..n]);
                    reply(tx, &req, E_OK, &out[..1 + n])
                }
                Err(e) => reply(tx, &req, e, &[]),
            }
        }
        _ => reply(tx, &req, E_INVAL, &[]),
    }
}

fn reply(tx: &mut [u8], req: &Request, status: i32, data: &[u8]) -> usize {
    encode_response_header(tx, req, (STATUS_LEN + data.len()) as u32);
    write_status(&mut tx[HDR_LEN..], status);
    tx[HDR_LEN + STATUS_LEN..][..data.len()].copy_from_slice(data);
    HDR_LEN + STATUS_LEN + data.len()
}

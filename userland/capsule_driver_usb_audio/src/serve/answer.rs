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

//! One request answered. Pure but for the queue, for the proofs.

use super::play::Play;
use super::reply::{output_status, status};
use crate::wire::{decode_request, refused, Request, E_AGAIN, E_INVAL, E_NODEV, E_OK};
use crate::wire::{HDR_LEN, MAX_PCM_CHUNK, OP_HEALTHCHECK, OP_OUTPUT_STATUS};
use crate::wire::{OP_STREAM_START, OP_STREAM_STOP, OP_WRITE_PCM};

/// Answers `msg` into `tx`; returns the reply's length. A frame that does
/// not decode is answered too: its caller waits in its call for one.
pub fn answer(play: &mut Play, msg: &[u8], tx: &mut [u8]) -> usize {
    let Some(req) = decode_request(msg) else {
        return status(tx, &refused(msg), E_INVAL);
    };
    if req.payload_len != 0 && req.op != OP_WRITE_PCM {
        return status(tx, &req, E_INVAL);
    }
    let code = match req.op {
        OP_HEALTHCHECK => E_OK,
        OP_OUTPUT_STATUS => return output_status(tx, &req, play.device.as_ref()),
        OP_STREAM_START if play.device.is_none() => E_NODEV,
        OP_STREAM_START => {
            play.running = true;
            E_OK
        }
        OP_STREAM_STOP => {
            play.running = false;
            play.queue.clear();
            E_OK
        }
        OP_WRITE_PCM => write_pcm(play, &req, msg),
        _ => E_INVAL,
    };
    status(tx, &req, code)
}

/// Queued whole or refused whole; E_AGAIN while the queue is too full, which
/// the audio server's pump retries.
fn write_pcm(play: &mut Play, req: &Request, msg: &[u8]) -> i32 {
    let want = req.payload_len as usize;
    let Some(pcm) = msg.get(HDR_LEN..HDR_LEN.saturating_add(want)) else {
        return E_INVAL;
    };
    if want == 0 || want > MAX_PCM_CHUNK {
        E_INVAL
    } else if play.device.is_none() {
        E_NODEV
    } else if !play.queue.push(pcm) {
        E_AGAIN
    } else {
        E_OK
    }
}

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

use nonos_libc::mk_ipc_send;

use super::ops;
use super::proto::{self, Request};
use super::pump::PumpState;
use super::streams::StreamTable;
use crate::mixer::Mixer;
use crate::sink::Sink;

const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_0019;

pub fn handle(
    msg: &[u8],
    mixer: &mut Mixer,
    sink: &Sink,
    table: &mut StreamTable,
    pump: &mut PumpState,
    tx: &mut [u8],
    sender: u32,
) {
    let req = match proto::decode(msg) {
        Some(r) => r,
        None => return refuse(msg, tx),
    };
    if req.op == proto::OP_OUTPUT_STATUS {
        let (code, flags) = output(sink);
        send(proto::encode_output_reply(&req, code, flags, tx), tx);
        return;
    }
    if req.op == proto::OP_SET_VOLUME {
        let status = ops::set_volume(&req, msg, pump);
        send(proto::encode_volume_reply(&req, status, pump.volume(), tx), tx);
        return;
    }
    if req.op == proto::OP_STREAM_OPEN {
        // A machine the driver cannot play on is refused at open, by name,
        // instead of taking a stream whose audio goes nowhere.
        let (status, id) = match output(sink) {
            (proto::OUTPUT_READY, _) | (proto::OUTPUT_NOT_ANSWERING, _) => {
                ops::stream_open(table, sender)
            }
            _ => (proto::E_NODEV, 0),
        };
        let n = proto::encode_open_reply(&req, status, id, tx);
        if n != 0 {
            let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), n);
        }
        return;
    }
    let status = route(&req, msg, mixer, sink, table, pump, sender);
    let n = proto::encode_reply(&req, status, tx);
    if n != 0 {
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), n);
    }
}

/// What the driver says this machine's audio is. A driver that does not
/// answer in time (still bringing the controller up, or wedged) is reported
/// as such and does not stop a stream from opening.
fn output(sink: &Sink) -> (u32, u32) {
    sink.output_status(0).unwrap_or((proto::OUTPUT_NOT_ANSWERING, 0))
}

fn send(n: usize, tx: &[u8]) {
    if n != 0 {
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), n);
    }
}

/// Answer a request while there is no driver to play on: the output status
/// says there is no sound hardware, a stream open and a volume change are
/// refused with E_NODEV so the client can ask why, and everything else is
/// E_INVAL. The volume a refusal names is the one a service starts with, as
/// nothing here plays to change it for.
pub fn no_sink(msg: &[u8], tx: &mut [u8]) {
    let Some(req) = proto::decode(msg) else { return refuse(msg, tx) };
    let n = match req.op {
        proto::OP_OUTPUT_STATUS => proto::encode_output_reply(&req, proto::OUTPUT_NO_DEVICE, 0, tx),
        proto::OP_STREAM_OPEN => proto::encode_open_reply(&req, proto::E_NODEV, 0, tx),
        proto::OP_SET_VOLUME => {
            proto::encode_volume_reply(&req, proto::E_NODEV, proto::MasterVolume::FULL, tx)
        }
        _ => proto::encode_reply(&req, proto::E_INVAL, tx),
    };
    send(n, tx);
}

/// Answer a frame nothing here can serve, one that does not decode, with
/// E_INVAL. Its caller is
/// blocked in its call until a reply comes, and the kernel holds the call's
/// place in this service's reply queue until then: a frame left unanswered
/// costs its caller the whole timeout and the queue a slot for as long as the
/// caller lives.
pub fn refuse(msg: &[u8], tx: &mut [u8]) {
    let n = proto::encode_reply(&proto::refused(msg), proto::E_INVAL, tx);
    if n != 0 {
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, tx.as_ptr(), n);
    }
}

fn route(
    req: &Request,
    msg: &[u8],
    mixer: &mut Mixer,
    sink: &Sink,
    table: &mut StreamTable,
    pump: &mut PumpState,
    sender: u32,
) -> i32 {
    // Tones go out at the master volume the streams play at.
    let master = pump.gain();
    match req.op {
        proto::OP_PLAY_TONE => ops::play_tone(req, msg, mixer, sink, master),
        proto::OP_PLAY_PCM => ops::play_pcm(req, msg, mixer, sink, master),
        proto::OP_STOP => {
            mixer.clear();
            proto::E_OK
        }
        proto::OP_FEED_PCM => ops::stream_feed(req, msg, table, pump, sink, sender),
        proto::OP_PAUSE => ops::stream_pause(req, msg, table, sender),
        proto::OP_RESUME => ops::stream_resume(req, msg, table, sender),
        proto::OP_CLOSE => ops::stream_close(req, msg, table, sender),
        _ => proto::E_INVAL,
    }
}

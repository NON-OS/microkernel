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

use nonos_libc::mk_idle_ms;

use crate::mark::mark;
use crate::mixer::Mixer;
use crate::server::proto::{E_AGAIN, E_OK};
use crate::server::pump::PumpState;
use crate::server::streams::StreamTable;
use crate::sink::Sink;
use nonos_audio_proto::TONE_MSG_LEN as TONE_MSG;

const CHUNK_BYTES: usize = 4096;
const HALF_PERIOD: usize = 44;
const AMPLITUDE: i16 = 0x1800;
const REPLY_MSG: usize = 24;
/// A period is about 43 ms; 50 waits of 10 ms outlast the queue draining.
const AGAIN_WAITS: u32 = 50;
const AGAIN_PAUSE_MS: u64 = 10;

/// The boot self-tests, in order. They leave the DAC running with nothing
/// queued; stopped here, it idles until a client plays and the pump starts it.
pub fn boot(mixer: &mut Mixer, sink: &Sink) {
    run_mix(mixer, sink);
    sink.stream_start(3);
    crate::selftest_stream::run_streams(sink);
    run(sink);
    sink.stream_stop(4);
}

/*
 * The stream test just before this one leaves the driver's queue full, and a
 * write to a full queue is answered E_AGAIN. That is the driver pacing its
 * client, not a broken sink, so the write is retried while the DAC drains;
 * only another answer, or a queue that never drains, is a fail.
 */
fn run(sink: &Sink) {
    let mut pcm = vec![0u8; CHUNK_BYTES];
    fill_tone(&mut pcm);
    let mut status = sink.write_pcm_status(&pcm, 1);
    let mut waits = 0;
    while status == E_AGAIN && waits < AGAIN_WAITS {
        let _ = mk_idle_ms(AGAIN_PAUSE_MS);
        waits += 1;
        status = sink.write_pcm_status(&pcm, 1);
    }
    if status == E_OK {
        mark("[AUDIO] sink-ok\n");
    } else {
        mark("[AUDIO] sink-fail\n");
    }
}

fn run_mix(mixer: &mut Mixer, sink: &Sink) {
    let mut tx = [0u8; REPLY_MSG];
    let mut table = StreamTable::new();
    let mut pump = PumpState::new();
    let mut a = [0u8; TONE_MSG];
    let na = tone_request(1, 440, 20, 0x2000, &mut a);
    crate::server::handle(&a[..na], mixer, sink, &mut table, &mut pump, &mut tx, 0);
    let mut b = [0u8; TONE_MSG];
    let nb = tone_request(2, 660, 20, 0x2000, &mut b);
    crate::server::handle(&b[..nb], mixer, sink, &mut table, &mut pump, &mut tx, 0);
}

fn tone_request(id: u32, freq: u32, ms: u32, gain: u16, out: &mut [u8]) -> usize {
    nonos_audio_proto::tone_request(out, id, freq, ms, gain)
}

fn fill_tone(buf: &mut [u8]) {
    let frames = buf.len() / 4;
    let mut i = 0usize;
    while i < frames {
        let sample = if (i / HALF_PERIOD) & 1 == 0 { AMPLITUDE } else { -AMPLITUDE };
        let le = sample.to_le_bytes();
        buf[i * 4] = le[0];
        buf[i * 4 + 1] = le[1];
        buf[i * 4 + 2] = le[0];
        buf[i * 4 + 3] = le[1];
        i += 1;
    }
}

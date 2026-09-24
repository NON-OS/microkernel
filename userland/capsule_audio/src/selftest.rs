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

use crate::mark::mark;
use crate::mixer::Mixer;
use crate::server::pump::PumpState;
use crate::server::streams::StreamTable;
use crate::sink::Sink;
use nonos_audio_proto::TONE_MSG_LEN as TONE_MSG;

const CHUNK_BYTES: usize = 4096;
const HALF_PERIOD: usize = 44;
const AMPLITUDE: i16 = 0x1800;
const REPLY_MSG: usize = 24;

pub fn run(sink: &Sink) {
    let mut pcm = vec![0u8; CHUNK_BYTES];
    fill_tone(&mut pcm);
    if sink.write_pcm(&pcm, 1) {
        mark("[AUDIO] sink-ok\n");
    } else {
        mark("[AUDIO] sink-fail\n");
    }
}

pub fn run_mix(mixer: &mut Mixer, sink: &Sink) {
    let mut tx = [0u8; REPLY_MSG];
    let mut table = StreamTable::new();
    let mut pump = PumpState::new();
    let mut a = [0u8; TONE_MSG];
    let na = tone_request(1, 440, 20, 0x2000, &mut a);
    crate::server::handle(&a[..na], mixer, sink, &mut table, &mut pump, &mut tx);
    let mut b = [0u8; TONE_MSG];
    let nb = tone_request(2, 660, 20, 0x2000, &mut b);
    crate::server::handle(&b[..nb], mixer, sink, &mut table, &mut pump, &mut tx);
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

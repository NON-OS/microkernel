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

//! Starting and stopping the output stream, and the bring-up tone, served
//! as the loop serves them: the engine runs with the outputs open, and a
//! stop leaves it stopped, the outputs closed and the queue empty.

use super::serving::{answer, ask, playing, tx, Playing};
use crate::audio::PcmQueue;
use crate::constants::{AMP_MUTE, AMP_OUT_UNMUTE, SDCTL_RUN, SD_CTL};
use crate::proofs::program_sim::{amp, at};
use crate::protocol::{E_OK, OP_PLAY_TONE, OP_STREAM_START, OP_STREAM_STOP};
use crate::server::handlers::play_tone;
use crate::server::handlers::stream::{handle_stream_start, handle_stream_stop};

fn runs(p: &Playing) -> bool {
    p.b.sim.bar.wrote8((p.d.stream_off + SD_CTL) as usize) & SDCTL_RUN != 0
}

#[test]
fn a_started_stream_plays_with_its_outputs_open_and_a_stop_closes_them() {
    let mut p = playing();
    let (mut tx, mut q, mut running) = (tx(), PcmQueue::new(), false);
    q.push(&[7; 64]);
    let from = p.b.sim.sent().len();
    handle_stream_start(&mut p.d, &ask(OP_STREAM_START, 0), &mut tx, &mut running);
    assert_eq!(answer(OP_STREAM_START).0, E_OK);
    assert!(running && runs(&p), "the engine is not running");
    at(&p.b.sim.sent()[from..], amp(0x14, AMP_OUT_UNMUTE));
    let from = p.b.sim.sent().len();
    handle_stream_stop(&mut p.d, &ask(OP_STREAM_STOP, 0), &mut tx, &mut q, &mut running);
    assert_eq!(answer(OP_STREAM_STOP).0, E_OK);
    assert!(!running && !runs(&p), "the engine still runs");
    at(&p.b.sim.sent()[from..], amp(0x14, AMP_OUT_UNMUTE | AMP_MUTE));
    assert_eq!(q.pop_into(&mut [0u8; 64]), 0, "the stopped stream's PCM was kept");
}

#[test]
fn the_tone_fills_the_ring_and_plays_until_a_stop() {
    let mut p = playing();
    let (mut played, mut running) = (true, false);
    play_tone::handle(&mut p.d, &ask(OP_PLAY_TONE, 0), &mut tx(), &mut played, &mut running);
    assert_eq!(answer(OP_PLAY_TONE).0, E_OK);
    assert!(running && runs(&p) && !played, "the tone is not playing");
    assert_ne!(p.b.pcm.wrote32(0), 0, "the ring holds no tone");
}

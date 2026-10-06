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

//! A track loads a tick at a time: each step reads a chunk or decodes a
//! block and stops when its budget is spent, the file is asked for in store
//! round trips never past one byte over the limit, the result is the same
//! waveform and a rewound decoder whatever the budget, and a file too large,
//! unreadable or not audio fails the load rather than hanging it.

use std::cell::Cell;
use std::rc::Rc;

use crate::decode::wav::WavDecoder;
use crate::decode::Decoder;
use crate::loading::{Load, Source, Step, BARS, CHUNK};
use crate::peaks::Peaks;
use crate::track_limit::{READ_LIMIT, TOO_LARGE};

/// A file in memory, handed out as the store would, recording each ask.
struct Mem {
    file: Vec<u8>,
    at: usize,
    asks: Vec<u32>,
    fail: bool,
}

impl Mem {
    fn new(file: Vec<u8>) -> Mem {
        Mem { file, at: 0, asks: Vec::new(), fail: false }
    }
}

impl Source for Mem {
    fn chunk(&mut self, want: u32, out: &mut Vec<u8>) -> Result<usize, &'static str> {
        self.asks.push(want);
        if self.fail {
            return Err("vfs read failed");
        }
        let end = (self.at + want as usize).min(self.file.len());
        out.extend_from_slice(&self.file[self.at..end]);
        let n = end - self.at;
        self.at = end;
        Ok(n)
    }
}

/// A source of zeros that never ends, as a file grown past the limit reads;
/// it counts what it was asked for where the test can read it afterwards.
struct Endless(Rc<Cell<u64>>);

impl Source for Endless {
    fn chunk(&mut self, want: u32, out: &mut Vec<u8>) -> Result<usize, &'static str> {
        assert!(want > 0 && want <= CHUNK, "asked for {want}");
        out.resize(out.len() + want as usize, 0);
        self.0.set(self.0.get() + want as u64);
        Ok(want as usize)
    }
}

fn open_wav(bytes: Vec<u8>) -> Result<Box<dyn Decoder>, &'static str> {
    WavDecoder::new(bytes).map(|d| Box::new(d) as Box<dyn Decoder>)
}

/// A RIFF/WAVE file of 16-bit mono samples.
fn wav(samples: &[i16]) -> Vec<u8> {
    let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
    let mut v = Vec::new();
    v.extend_from_slice(b"RIFF");
    v.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
    v.extend_from_slice(b"WAVEfmt ");
    v.extend_from_slice(&16u32.to_le_bytes());
    v.extend_from_slice(&1u16.to_le_bytes());
    v.extend_from_slice(&1u16.to_le_bytes());
    v.extend_from_slice(&8000u32.to_le_bytes());
    v.extend_from_slice(&16000u32.to_le_bytes());
    v.extend_from_slice(&2u16.to_le_bytes());
    v.extend_from_slice(&16u16.to_le_bytes());
    v.extend_from_slice(b"data");
    v.extend_from_slice(&(data.len() as u32).to_le_bytes());
    v.extend_from_slice(&data);
    v
}

fn samples(n: usize) -> Vec<i16> {
    (0..n).map(|i| ((i * 37) % 2000) as i16 - 1000).collect()
}

/// Step with `units` of work per step until the load ends; the steps taken.
fn run<S: Source>(load: &mut Load<S>, units: usize) -> (Step, usize) {
    let mut steps = 0;
    loop {
        steps += 1;
        let mut left = units;
        let step = load.step(&mut || {
            left = left.saturating_sub(1);
            left > 0
        });
        if !matches!(step, Step::More) {
            return (step, steps);
        }
        assert!(steps < 100_000, "the load never ended");
    }
}

fn ready(step: Step) -> (Box<dyn Decoder>, Vec<u8>) {
    match step {
        Step::Ready(dec, bars) => (dec, bars),
        Step::More => panic!("not done"),
        Step::Failed(why) => panic!("failed: {why}"),
    }
}

#[test]
fn one_unit_a_step_still_loads_the_whole_track() {
    let pcm = samples(200_000);
    let file = wav(&pcm);
    let size = file.len() as u64;
    let mut load = Load::new(Mem::new(file), Some(size), open_wav).expect("fits");
    let (step, steps) = run(&mut load, 1);
    let (mut dec, bars) = ready(step);
    assert_eq!(bars.len(), BARS);
    // Every chunk and every 4096-sample block took a step of its own.
    assert!(steps > pcm.len() / 4096, "{steps} steps");
    // Rewound: the decoder plays the track from its first sample.
    let mut first = [0i16; 4];
    assert_eq!(dec.next(&mut first), 4);
    assert_eq!(first, pcm[..4]);
}

#[test]
fn the_waveform_does_not_depend_on_the_budget() {
    let pcm = samples(150_000);
    let mut whole = Peaks::default();
    whole.push(&pcm);
    let expect = whole.finish(BARS);
    for units in [1, 3, 64, usize::MAX] {
        let file = wav(&pcm);
        let mut load = Load::new(Mem::new(file), None, open_wav).expect("fits");
        let (_, bars) = ready(run(&mut load, units).0);
        assert_eq!(bars, expect, "budget {units}");
    }
}

#[test]
fn a_file_without_end_is_read_one_byte_past_the_limit_and_refused() {
    let asked = Rc::new(Cell::new(0));
    let mut endless = Load::new(Endless(asked.clone()), None, open_wav).expect("unknown size");
    match run(&mut endless, 16).0 {
        Step::Failed(why) => assert_eq!(why, TOO_LARGE),
        _ => panic!("a file past the limit loaded"),
    }
    // In store round trips of a chunk at most, and never past the one byte
    // over the limit that tells a long file from one that fits.
    assert_eq!(asked.get(), READ_LIMIT as u64);
}

#[test]
fn every_ask_is_at_most_a_chunk() {
    struct Asks(Mem);
    impl Source for Asks {
        fn chunk(&mut self, want: u32, out: &mut Vec<u8>) -> Result<usize, &'static str> {
            assert!(want <= CHUNK && want > 0);
            self.0.chunk(want, out)
        }
    }
    let mut load = Load::new(Asks(Mem::new(wav(&samples(90_000)))), None, open_wav).expect("fits");
    let _ = ready(run(&mut load, 2).0);
}

#[test]
fn a_size_past_the_limit_is_refused_before_reading() {
    let refused = Load::new(Mem::new(Vec::new()), Some(READ_LIMIT as u64), open_wav);
    assert_eq!(refused.err(), Some(TOO_LARGE));
}

#[test]
fn a_read_error_or_a_file_not_audio_fails_the_load() {
    let mut broken = Mem::new(wav(&samples(10)));
    broken.fail = true;
    let mut load = Load::new(broken, None, open_wav).expect("fits");
    assert!(matches!(run(&mut load, 4).0, Step::Failed("vfs read failed")));
    let mut text = Load::new(Mem::new(b"not a wav at all".to_vec()), None, open_wav).expect("fits");
    assert!(matches!(run(&mut text, 4).0, Step::Failed(_)));
    // Over is over: a further step does not start again.
    assert!(matches!(text.step(&mut || true), Step::Failed(_)));
}

#[test]
fn progress_counts_the_read_then_stops() {
    let file = wav(&samples(300_000));
    let size = file.len() as u64;
    let mut load = Load::new(Mem::new(file), Some(size), open_wav).expect("fits");
    assert_eq!(load.percent_read(), Some(0));
    let _ = load.step(&mut || false);
    let first = load.percent_read().expect("reading");
    assert!(first > 0 && first < 100, "{first}");
    let (_, _) = ready(run(&mut load, usize::MAX).0);
    assert_eq!(load.percent_read(), None);
}

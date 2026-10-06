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

//! What the music player says about time, and what it lets a person find:
//! times at the track's own rate, an MP3's length and seek from the load's
//! pass, the playing row's meter from the real output, the Search page's
//! keys, and no invented artist.

use std::cell::Cell;
use std::rc::Rc;

use nonos_app_skeleton::{KEY_BACKSPACE, KEY_ENTER, KEY_ESC, KEY_LEFT};

use crate::audio_ui::search_key::{search_key, SearchKey, QUERY_MAX};
use crate::decode::mp3_index::{FrameIndex, SeekPlan, PRIME};
use crate::decode::{AudioInfo, Decoder};
use crate::library::track::Track;
use crate::transport::defs::{Fed, FeedSink};
use crate::transport::machine::Transport;

/// `frames` stereo frames at `rate`, every sample `amp`.
struct Tone {
    rate: u32,
    frames: usize,
    amp: i16,
}

impl Decoder for Tone {
    fn info(&self) -> AudioInfo {
        AudioInfo { rate: self.rate, channels: 2, total_frames: Some(self.frames as u64) }
    }
    fn kind(&self) -> &'static str {
        "WAV"
    }
    fn next(&mut self, out: &mut [i16]) -> usize {
        let n = out.len().min(self.frames * 2);
        out[..n].fill(self.amp);
        self.frames -= n / 2;
        n
    }
    fn seek(&mut self, _frame: u64) -> bool {
        true
    }
}

struct Sink(Rc<Cell<usize>>);

impl FeedSink for Sink {
    fn open(&mut self, _format: u16) -> Result<(), &'static str> {
        Ok(())
    }
    // One block a pump, as a server with a full queue answers: the track
    // stays playing between pumps.
    fn feed(&mut self, _pcm: &[i16]) -> Fed {
        self.0.set(self.0.get() + 1);
        if self.0.get() % 2 == 1 {
            Fed::Accepted
        } else {
            Fed::WouldBlock
        }
    }
    fn pause(&mut self) {}
    fn resume(&mut self) {}
    fn close(&mut self) {}
}

fn opened(rate: u32, frames: usize, amp: i16) -> Transport {
    let mut t = Transport::new(Box::new(Sink(Rc::new(Cell::new(0)))));
    assert!(t.open(Box::new(Tone { rate, frames, amp })).is_ok());
    t
}

#[test]
fn a_44k_track_reads_its_own_length() {
    let t = opened(44_100, 44_100 * 3, 1000);
    assert_eq!(t.frames_to_ms(t.dur_frames()), 3000, "three seconds, not 2756 ms");
    assert_eq!(t.secs_to_frames(10), 441_000);
}

#[test]
fn a_ten_second_skip_is_ten_seconds_of_the_track() {
    let mut t = opened(22_050, 22_050 * 60, 1000);
    assert!(t.seek_frames(t.secs_to_frames(10)));
    assert_eq!(t.frames_to_ms(t.pos_frames()), 10_000);
}

#[test]
fn the_row_meter_follows_what_is_sent() {
    let mut t = opened(48_000, 48_000, 12_000);
    assert_eq!(t.level(), 0, "nothing playing, nothing lit");
    t.play();
    t.pump();
    assert_eq!(t.level(), 12_000);
    t.pause();
    assert_eq!(t.level(), 0);
    let mut quiet = opened(48_000, 48_000, 0);
    quiet.play();
    quiet.pump();
    assert_eq!(quiet.level(), 0, "silence lights nothing");
}

fn indexed(frames: &[(usize, u64)], end: bool) -> FrameIndex {
    let mut ix = FrameIndex::default();
    for &(at, n) in frames {
        ix.record(at, n);
    }
    if end {
        ix.finish();
    }
    ix
}

#[test]
fn an_mp3_length_is_known_only_after_the_whole_pass() {
    let frames = [(0, 1152), (417, 1152), (835, 1152)];
    assert_eq!(indexed(&frames, false).total(), None);
    assert_eq!(indexed(&frames, true).total(), Some(3456));
    // A replay after the pass adds nothing.
    let mut ix = indexed(&frames, true);
    ix.record(0, 1152);
    ix.record(1252, 1152);
    assert_eq!(ix.total(), Some(3456));
}

#[test]
fn a_seek_lands_in_its_frame_primed_from_before() {
    let frames: Vec<(usize, u64)> = (0..10).map(|i| (i * 400, 1152)).collect();
    let ix = indexed(&frames, true);
    let target = 5 * 1152 + 100;
    assert_eq!(
        ix.plan(target),
        Some(SeekPlan { prime_at: (5 - PRIME) * 400, prime: PRIME, frame_at: 2000, skip: 100 })
    );
    // The first frames have fewer before them to prime from.
    assert_eq!(ix.plan(10), Some(SeekPlan { prime_at: 0, prime: 0, frame_at: 0, skip: 10 }));
    assert_eq!(ix.plan(1152).map(|p| p.prime), Some(1));
}

#[test]
fn no_seek_before_the_length_is_known() {
    let ix = indexed(&[(0, 1152), (400, 1152)], false);
    assert_eq!(ix.plan(500), None);
    assert_eq!(FrameIndex::default().plan(0), None);
}

#[test]
fn search_keys_edit_the_query() {
    let mut q = String::new();
    for c in b"bo t" {
        assert_eq!(search_key(&mut q, *c as u32), SearchKey::Edited);
    }
    assert_eq!(q, "bo t", "a space is typed, not play");
    assert_eq!(search_key(&mut q, KEY_BACKSPACE), SearchKey::Edited);
    assert_eq!(q, "bo ");
    assert_eq!(search_key(&mut q, KEY_ENTER), SearchKey::PlayFirst);
    assert_eq!(search_key(&mut q, KEY_ESC), SearchKey::Edited);
    assert!(q.is_empty());
    // With nothing to clear or erase, the keys are the transport's.
    assert_eq!(search_key(&mut q, KEY_ESC), SearchKey::Pass);
    assert_eq!(search_key(&mut q, KEY_BACKSPACE), SearchKey::Pass);
    assert_eq!(search_key(&mut q, KEY_LEFT), SearchKey::Pass);
}

#[test]
fn a_full_query_takes_no_more_and_still_swallows_text() {
    let mut q = "x".repeat(QUERY_MAX);
    assert_eq!(search_key(&mut q, b' ' as u32), SearchKey::Edited);
    assert_eq!(q.len(), QUERY_MAX);
}

#[test]
fn a_track_names_no_made_up_artist() {
    let t = Track::from_path("/audio/boot_tone.wav");
    assert_eq!(t.title, "Boot Tone");
    assert_eq!(t.artist, "boot_tone.wav");
    assert_eq!(t.dur_ms, 0, "unknown until it has loaded");
}

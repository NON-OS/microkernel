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

//! The audio player with no audio output: play stops at the first failed feed
//! and keeps the reason, instead of sitting in Playing with the position
//! frozen (or, against a service that stopped answering, blocking a reply
//! timeout on every tick), and the window's line for it names the cause.

use std::cell::Cell;
use std::rc::Rc;

use crate::audio_trouble::{library_trouble, notice, output_trouble, track_trouble, NO_SERVICE};
use crate::decode::{AudioInfo, Decoder};
use crate::transport::defs::{Fed, FeedSink, State};
use crate::transport::machine::Transport;

/// A track of `frames` stereo frames at the output rate.
struct Tone {
    frames: usize,
}

impl Decoder for Tone {
    fn info(&self) -> AudioInfo {
        AudioInfo { rate: 48_000, channels: 2, total_frames: Some(self.frames as u64) }
    }
    fn kind(&self) -> &'static str {
        "WAV"
    }
    fn next(&mut self, out: &mut [i16]) -> usize {
        let n = out.len().min(self.frames * 2);
        out[..n].fill(1000);
        self.frames -= n / 2;
        n
    }
}

/// A sink that answers every feed with `answer` and counts the feeds.
struct Sink {
    answer: fn() -> Fed,
    feeds: Rc<Cell<usize>>,
}

impl FeedSink for Sink {
    fn open(&mut self, _format: u16) -> Result<(), &'static str> {
        Ok(())
    }
    fn feed(&mut self, _pcm: &[i16]) -> Fed {
        self.feeds.set(self.feeds.get() + 1);
        (self.answer)()
    }
    fn pause(&mut self) {}
    fn resume(&mut self) {}
    fn close(&mut self) {}
}

fn transport(answer: fn() -> Fed) -> (Transport, Rc<Cell<usize>>) {
    let feeds = Rc::new(Cell::new(0));
    let mut t = Transport::new(Box::new(Sink { answer, feeds: feeds.clone() }));
    assert!(t.open(Box::new(Tone { frames: 48_000 })).is_ok());
    (t, feeds)
}

fn no_service() -> Fed {
    Fed::Failed(NO_SERVICE)
}

fn silent_service() -> Fed {
    Fed::Failed("audio.server feed: no reply")
}

#[test]
fn play_with_no_audio_service_stops_at_the_first_feed_and_says_why() {
    let (mut t, feeds) = transport(no_service);
    t.play();
    t.pump();
    assert!(t.state() == State::Paused);
    assert_eq!(t.fault(), Some(NO_SERVICE));
    assert_eq!(feeds.get(), 1);
}

#[test]
fn a_failed_sink_is_not_fed_again_until_play_is_pressed() {
    let (mut t, feeds) = transport(silent_service);
    t.play();
    for _ in 0..50 {
        t.pump();
    }
    assert_eq!(feeds.get(), 1);
    t.play();
    assert_eq!(t.fault(), None);
    t.pump();
    assert_eq!(feeds.get(), 2);
    assert!(t.state() == State::Paused);
}

#[test]
fn a_failed_sink_pauses_rather_than_ending_the_track() {
    // A stop reads as the end of the track and moves the queue on.
    let (mut t, _) = transport(silent_service);
    t.play();
    t.pump();
    assert!(t.state() != State::Stopped);
}

#[test]
fn a_sink_that_would_block_is_waited_on_without_a_fault() {
    let (mut t, _) = transport(|| Fed::WouldBlock);
    t.play();
    t.pump();
    assert!(t.state() == State::Playing);
    assert_eq!(t.fault(), None);
}

#[test]
fn a_track_played_to_its_end_leaves_no_fault() {
    let (mut t, _) = transport(|| Fed::Accepted);
    t.play();
    for _ in 0..1000 {
        t.pump();
        if t.state() == State::Stopped {
            break;
        }
    }
    assert!(t.state() == State::Stopped);
    assert_eq!(t.fault(), None);
}

#[test]
fn opening_the_next_track_clears_the_last_fault() {
    let (mut t, _) = transport(no_service);
    t.play();
    t.pump();
    assert!(t.fault().is_some());
    assert!(t.open(Box::new(Tone { frames: 10 })).is_ok());
    assert_eq!(t.fault(), None);
}

#[test]
fn every_output_fault_has_a_line_that_says_there_is_no_output() {
    assert_eq!(output_trouble(NO_SERVICE), NO_SERVICE);
    for fault in [
        "audio.server feed: no reply",
        "audio.server open: no reply",
        "audio.server feed rejected",
        "audio.server open rejected",
        "audio.server feed: invalid pcm length",
    ] {
        assert!(output_trouble(fault).starts_with("No audio output"));
    }
    assert_ne!(
        output_trouble("audio.server feed: no reply"),
        output_trouble("audio.server feed rejected")
    );
}

#[test]
fn a_stop_for_a_track_that_will_not_load_leaves_nothing_to_play() {
    // load_track stops the transport before it reads the new file, so a file
    // that then fails leaves no decoder: play cannot sound the track before.
    let (mut t, feeds) = transport(|| Fed::Accepted);
    t.stop();
    t.play();
    t.pump();
    assert!(t.state() == State::Stopped);
    assert_eq!(feeds.get(), 0);
}

#[test]
fn a_track_that_will_not_load_says_why_in_words() {
    let undecodable = track_trouble("unknown audio format");
    assert_eq!(undecodable, "Cannot play: only MP3 and WAV files can be decoded");
    assert_eq!(track_trouble("vfs ipc failed"), "Cannot play: the file store did not answer");
    for err in ["vfs open failed", "vfs read failed", "vfs path invalid"] {
        assert_eq!(track_trouble(err), "Cannot play: the file could not be read");
    }
    for err in ["bad riff header", "no mp3 frame"] {
        assert_eq!(track_trouble(err), "Cannot play: the file is damaged or not audio");
    }
}

#[test]
fn a_stream_the_service_would_not_open_reads_as_no_output() {
    assert!(track_trouble("audio.server open rejected").starts_with("No audio output"));
    assert!(track_trouble("audio.server open: no reply").starts_with("No audio output"));
}

#[test]
fn an_unlistable_music_folder_is_not_called_empty() {
    let failed = library_trouble(Some("vfs ipc failed"), 0);
    let refused = library_trouble(Some("vfs list failed"), 0);
    let empty = library_trouble(None, 0);
    assert_eq!(failed, Some("No music: the file store did not answer"));
    assert_eq!(refused, Some("No music: the folder /home/nonos/music could not be read"));
    assert_eq!(
        empty,
        Some("No music yet: paste an MP3 link into Search, or put files in /home/nonos/music")
    );
}

#[test]
fn a_library_with_tracks_has_nothing_to_say() {
    assert_eq!(library_trouble(None, 3), None);
}

#[test]
fn the_bar_names_the_most_specific_trouble() {
    let track = Some("Cannot play: the file could not be read");
    let output = Some(NO_SERVICE);
    let library = Some("No music yet: add MP3 or WAV files to /audio");
    assert_eq!(notice(track, output, library), track);
    assert_eq!(notice(None, output, library), output);
    assert_eq!(notice(None, None, library), library);
    assert_eq!(notice(None, None, None), None);
}

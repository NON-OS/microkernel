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

//! The audio player's stream on audio.server goes back when the window that
//! owns it closes (its app, and the transport with it, is dropped) and when
//! play ends with nothing after it, against a server that, as capsule_audio's
//! stream table does, holds at most two streams for one process.

use std::cell::RefCell;
use std::rc::Rc;

use crate::audio_trouble::output_trouble;
use crate::decode::{AudioInfo, Decoder};
use crate::transport::defs::{Fed, FeedSink, State};
use crate::transport::machine::Transport;

/// The server's per-process limit (capsule_audio's `PER_OWNER`).
const PER_OWNER: usize = 2;

#[derive(Default)]
struct Server {
    open: usize,
    opened: usize,
    closed: usize,
}

/// A client of the shared server, as AudioClient is: one stream at most, and
/// a close without one sends nothing.
struct Client {
    server: Rc<RefCell<Server>>,
    held: bool,
}

impl FeedSink for Client {
    fn open(&mut self, _format: u16) -> Result<(), &'static str> {
        let mut s = self.server.borrow_mut();
        if s.open >= PER_OWNER {
            return Err("audio.server open rejected");
        }
        s.open += 1;
        s.opened += 1;
        self.held = true;
        Ok(())
    }
    fn feed(&mut self, _pcm: &[i16]) -> Fed {
        if self.held {
            Fed::Accepted
        } else {
            Fed::Failed("audio.server feed rejected")
        }
    }
    fn pause(&mut self) {}
    fn resume(&mut self) {}
    fn close(&mut self) {
        if self.held {
            let mut s = self.server.borrow_mut();
            s.open -= 1;
            s.closed += 1;
            self.held = false;
        }
    }
}

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
        out[..n].fill(500);
        self.frames -= n / 2;
        n
    }
}

fn tone() -> Box<dyn Decoder> {
    Box::new(Tone { frames: 2048 })
}

/// A window's transport, as PlayerApp::new builds it, with its first track.
fn window(server: &Rc<RefCell<Server>>) -> Transport {
    let mut t = Transport::new(Box::new(Client { server: server.clone(), held: false }));
    assert_eq!(t.open(tone()), Ok(()));
    t
}

fn play_to_end(t: &mut Transport) {
    t.play();
    for _ in 0..100 {
        t.pump();
        if t.state() == State::Stopped {
            return;
        }
    }
    panic!("the track did not end");
}

#[test]
fn a_third_window_of_one_process_still_gets_sound() {
    // The boot instance never exits: each window that closes drops its app.
    let server = Rc::new(RefCell::new(Server::default()));
    for _ in 0..3 {
        let mut t = window(&server);
        play_to_end(&mut t);
        assert_eq!(t.fault(), None);
        drop(t);
        assert_eq!(server.borrow().open, 0);
    }
    assert_eq!(server.borrow().opened, 3);
    assert_eq!(server.borrow().closed, 3);
}

#[test]
fn a_closed_window_gives_its_stream_back_while_paused_too() {
    let server = Rc::new(RefCell::new(Server::default()));
    let mut t = window(&server);
    t.play();
    t.pause();
    drop(t);
    assert_eq!(server.borrow().open, 0);
}

#[test]
fn a_window_holds_one_stream_however_many_tracks_it_plays() {
    let server = Rc::new(RefCell::new(Server::default()));
    let mut t = window(&server);
    for _ in 0..5 {
        assert_eq!(t.open(tone()), Ok(()));
        assert_eq!(server.borrow().open, 1);
    }
}

#[test]
fn a_released_stream_is_opened_fresh_by_play() {
    let server = Rc::new(RefCell::new(Server::default()));
    let mut t = window(&server);
    play_to_end(&mut t);
    t.release();
    assert_eq!(server.borrow().open, 0);
    t.release();
    assert_eq!(server.borrow().closed, 1);
    t.play();
    assert_eq!(server.borrow().open, 1);
    assert_eq!(server.borrow().opened, 2);
    assert!(t.state() == State::Playing);
    assert_eq!(t.fault(), None);
}

#[test]
fn a_new_track_after_a_release_opens_one_stream() {
    let server = Rc::new(RefCell::new(Server::default()));
    let mut t = window(&server);
    t.release();
    assert_eq!(t.open(tone()), Ok(()));
    assert_eq!(server.borrow().open, 1);
    t.stop();
    assert_eq!(server.borrow().open, 0);
    drop(t);
    assert_eq!(server.borrow().closed, 2);
}

#[test]
fn a_stream_the_server_refuses_on_play_pauses_with_the_reason() {
    let server = Rc::new(RefCell::new(Server::default()));
    let mut t = window(&server);
    t.release();
    server.borrow_mut().open = PER_OWNER;
    t.play();
    assert!(t.state() == State::Paused);
    let fault = t.fault().expect("a refused stream is a fault");
    assert_eq!(output_trouble(fault), "No audio output: the audio service refused the sound");
    server.borrow_mut().open = 0;
    t.play();
    assert!(t.state() == State::Playing);
    assert_eq!(server.borrow().open, 1);
}

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

//! A proxy on the far side of the carrier, speaking what net.socks5 and
//! net.anon speak: a reset, numbered frames whose last answer is kept and
//! given again to the same number with the same or no bytes, the SOCKS5
//! greeting and CONNECT, and a relay whose far end answers only after a
//! number of polls. It keeps its own clock, so every wait the tunnel makes
//! is measured, and it can lose, split, flood and garble its answers.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::carrier::Carrier;

/// A fake proxy that outlives the tunnel using it, so what the tunnel sends
/// as it is dropped, and the time it spent, can still be looked at.
#[derive(Clone)]
pub struct Shared(pub Rc<RefCell<FakeProxy>>);

impl Shared {
    pub fn new(fake: FakeProxy) -> Shared {
        Shared(Rc::new(RefCell::new(fake)))
    }
}

impl Carrier for Shared {
    fn call(&mut self, frame: &[u8], wait_ms: u64, into: &mut [u8]) -> i64 {
        self.0.borrow_mut().call(frame, wait_ms, into)
    }

    fn now_ms(&self) -> i64 {
        self.0.borrow().now
    }

    fn pause(&mut self, ms: u64) {
        self.0.borrow_mut().pause(ms)
    }
}

/// Bytes the far end sends once the request has gone out, after this many
/// polls have asked for them.
pub struct Delivery {
    pub after_polls: u32,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Stage {
    Greeting,
    Request,
    Connecting(u32),
    Relay,
    Over,
}

pub struct FakeProxy {
    pub now: i64,
    pub calls: usize,
    pub frames: Vec<Vec<u8>>,
    pub resets: usize,
    /// The method the greeting is answered with: 0, or 0xFF when full.
    pub method: u8,
    /// The CONNECT's reply code.
    pub rep: u8,
    /// Polls answered with nothing before the CONNECT reply, as net.anon
    /// waits for its exit to connect.
    pub connect_polls: u32,
    /// The CONNECT reply comes in two answers.
    pub split_reply: bool,
    pub deliveries: VecDeque<Delivery>,
    /// The far end closes with its last delivery.
    pub close_after: bool,
    /// The next answer that delivers bytes is lost on its way back once.
    pub lose_next_delivery: bool,
    /// The next answer to a write is lost on its way back once.
    pub lose_next_write: bool,
    /// Never answers at all.
    pub dead: bool,
    /// Once open, answers given verbatim: what the call returns, and the
    /// bytes written into the caller's buffer.
    pub raw: VecDeque<(i64, Vec<u8>)>,
    /// Bytes of noise every relay answer carries.
    pub flood: usize,
    pub host: Vec<u8>,
    pub port: u16,
    /// What the exit was given to send to the host.
    pub to_exit: Vec<u8>,
    pub(crate) stage: Stage,
    pub(crate) kept: Option<(u32, Vec<u8>, Vec<u8>)>,
    pub(crate) split_left: Vec<u8>,
    pub(crate) written: bool,
}

impl FakeProxy {
    /// net.socks5's shape: the CONNECT is answered at once.
    pub fn nym() -> FakeProxy {
        FakeProxy {
            now: 1_000,
            calls: 0,
            frames: Vec::new(),
            resets: 0,
            method: 0,
            rep: 0,
            connect_polls: 0,
            split_reply: false,
            deliveries: VecDeque::new(),
            close_after: false,
            lose_next_delivery: false,
            lose_next_write: false,
            dead: false,
            raw: VecDeque::new(),
            flood: 0,
            host: Vec::new(),
            port: 0,
            to_exit: Vec::new(),
            stage: Stage::Greeting,
            kept: None,
            split_left: Vec::new(),
            written: false,
        }
    }

    /// net.anon's shape: the CONNECT is answered once the exit connects.
    pub fn anon(polls: u32) -> FakeProxy {
        FakeProxy { connect_polls: polls, ..FakeProxy::nym() }
    }

    pub fn deliver(mut self, after_polls: u32, bytes: &[u8]) -> FakeProxy {
        self.deliveries.push_back(Delivery { after_polls, bytes: bytes.to_vec() });
        self
    }

    /// The numbered frames sent, as (number, bytes).
    pub fn numbered(&self) -> Vec<(u32, Vec<u8>)> {
        self.frames
            .iter()
            .filter(|f| f.first() == Some(&2) && f.len() >= 5)
            .map(|f| (u32::from_le_bytes([f[1], f[2], f[3], f[4]]), f[5..].to_vec()))
            .collect()
    }

    fn serve(&mut self, body: &[u8]) -> (Vec<u8>, bool, bool) {
        match self.stage {
            Stage::Greeting => {
                if body != [5, 1, 0] {
                    self.stage = Stage::Over;
                    return (vec![1], false, false);
                }
                if self.method == 0xFF {
                    self.stage = Stage::Over;
                    return (vec![1, 5, 0xFF], false, false);
                }
                self.stage = Stage::Request;
                (vec![0, 5, self.method], false, false)
            }
            Stage::Request => {
                let len = body[4] as usize;
                self.host = body[5..5 + len].to_vec();
                self.port = u16::from_be_bytes([body[5 + len], body[6 + len]]);
                if self.connect_polls > 0 {
                    self.stage = Stage::Connecting(self.connect_polls);
                    return (vec![0], false, false);
                }
                (self.connect_answer(), false, false)
            }
            Stage::Connecting(n) if n > 1 => {
                self.stage = Stage::Connecting(n - 1);
                (vec![0], false, false)
            }
            Stage::Connecting(_) => (self.connect_answer(), false, false),
            Stage::Relay => self.relay(body),
            Stage::Over => (vec![1], false, false),
        }
    }

    fn connect_answer(&mut self) -> Vec<u8> {
        let reply = [5, self.rep, 0, 1, 0, 0, 0, 0, 0, 0];
        if self.rep != 0 {
            self.stage = Stage::Over;
            return [&[1u8][..], &reply[..]].concat();
        }
        self.stage = Stage::Relay;
        if self.split_reply {
            self.split_left = reply[3..].to_vec();
            return [&[0u8][..], &reply[..3]].concat();
        }
        [&[0u8][..], &reply[..]].concat()
    }

    /// The answer, whether it delivered bytes, and whether it took a write.
    fn relay(&mut self, body: &[u8]) -> (Vec<u8>, bool, bool) {
        let mut out = vec![0u8];
        if !self.split_left.is_empty() {
            out.extend(std::mem::take(&mut self.split_left));
            return (out, false, false);
        }
        out.extend(std::iter::repeat_n(0xAB, self.flood));
        if !body.is_empty() {
            self.to_exit.extend_from_slice(body);
            self.written = true;
            return (out, false, true);
        }
        if !self.written {
            return (out, false, false);
        }
        let Some(head) = self.deliveries.front_mut() else {
            return (out, false, false);
        };
        if head.after_polls > 0 {
            head.after_polls -= 1;
            return (out, false, false);
        }
        let Some(d) = self.deliveries.pop_front() else {
            return (out, false, false);
        };
        out.extend(d.bytes);
        if self.deliveries.is_empty() && self.close_after {
            out[0] = 1;
            self.stage = Stage::Over;
        }
        (out, true, false)
    }
}

impl Carrier for FakeProxy {
    fn call(&mut self, frame: &[u8], wait_ms: u64, into: &mut [u8]) -> i64 {
        self.calls += 1;
        self.now += 1;
        self.frames.push(frame.to_vec());
        if self.dead {
            self.now += wait_ms as i64;
            return -1;
        }
        if self.stage == Stage::Relay || self.stage == Stage::Over {
            if let Some((got, bytes)) = self.raw.pop_front() {
                let n = bytes.len().min(into.len());
                into[..n].copy_from_slice(&bytes[..n]);
                return got;
            }
        }
        let (answer, delivered, wrote) = match frame.first() {
            Some(1) => {
                self.resets += 1;
                self.stage = Stage::Greeting;
                self.kept = None;
                self.written = false;
                self.split_left.clear();
                (vec![0], false, false)
            }
            Some(2) if frame.len() >= 5 => {
                let seq = u32::from_le_bytes([frame[1], frame[2], frame[3], frame[4]]);
                let body = frame[5..].to_vec();
                match &self.kept {
                    Some((s, b, a)) if *s == seq && (body.is_empty() || *b == body) => {
                        (a.clone(), false, false)
                    }
                    _ => {
                        let (a, delivered, wrote) = self.serve(&body);
                        self.kept = Some((seq, body, a.clone()));
                        (a, delivered, wrote)
                    }
                }
            }
            _ => (vec![1], false, false),
        };
        let lost = (delivered && std::mem::take(&mut self.lose_next_delivery))
            || (wrote && std::mem::take(&mut self.lose_next_write));
        if lost {
            self.now += wait_ms as i64;
            return -1;
        }
        let n = answer.len().min(into.len());
        into[..n].copy_from_slice(&answer[..n]);
        answer.len() as i64
    }

    fn now_ms(&self) -> i64 {
        self.now
    }

    fn pause(&mut self, ms: u64) {
        self.now += ms as i64;
    }
}

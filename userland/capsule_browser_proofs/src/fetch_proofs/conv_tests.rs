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

//! A conversation with a proxy that answers late, loses answers and turns
//! calls away. A frame left unanswered is asked again unchanged, nothing
//! overtakes it, and every byte reaches the exit once and in order.

use crate::browser::net::mixnet::conv::{Conv, Refusal, UNSENT_MAX};
use crate::browser::net::mixnet::frames::{numbered, reset, CARRY_MAX, FIRST_SEQ};

/// The stream the conversations here name.
const S: u32 = 3;

/// A proxy by the rule in capsule_socks5 server/kept.rs: the kept answer
/// for a number asked again with no bytes or the same bytes; new bytes
/// under that number carried, the missed answer in front of theirs.
#[derive(Default)]
struct Proxy {
    kept: Option<(u32, Vec<u8>, Vec<u8>)>,
    /// What reached the exit, in order.
    exit: Vec<u8>,
    /// What the exit sent back that no answer has carried yet.
    back: Vec<u8>,
    resets: u32,
}

impl Proxy {
    fn serve(&mut self, frame: &[u8]) -> Vec<u8> {
        match frame[0] {
            1 | 3 => {
                self.kept = None;
                self.resets += 1;
                vec![0]
            }
            2 | 4 => {
                let at = if frame[0] == 4 { 5 } else { 1 };
                let seq = u32::from_le_bytes([frame[at], frame[at + 1], frame[at + 2], frame[at + 3]]);
                let body = &frame[at + 4..];
                let out = match &self.kept {
                    Some((s, carried, reply)) if *s == seq && (body.is_empty() || carried == body) => {
                        return reply.clone();
                    }
                    Some((s, _, reply)) if *s == seq => {
                        let mut out = reply.clone();
                        self.exit.extend_from_slice(body);
                        out.append(&mut self.back);
                        out
                    }
                    _ => {
                        self.exit.extend_from_slice(body);
                        let mut out = vec![0];
                        out.append(&mut self.back);
                        out
                    }
                };
                self.kept = Some((seq, body.to_vec(), out.clone()));
                out
            }
            other => panic!("a frame no proxy speaks: {other}"),
        }
    }
}

/// What became of one call.
#[derive(Clone, Copy)]
enum Fate {
    Answered,
    /// The proxy served it; the answer came after the caller stopped waiting.
    Late,
    /// The kernel turned it away (EBUSY): the proxy never saw it.
    Busy,
}

/// A small deterministic generator, so a failing seed can be replayed.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn fate(&mut self) -> Fate {
        match self.next() % 4 {
            0 => Fate::Late,
            1 => Fate::Busy,
            _ => Fate::Answered,
        }
    }
}

/// One call carrying the conversation's next frame, with `fate`.
fn call(conv: &mut Conv, proxy: &mut Proxy, fate: Fate) {
    let frame = conv.frame();
    match fate {
        Fate::Busy => {}
        Fate::Late => {
            proxy.serve(&frame);
        }
        Fate::Answered => {
            let reply = proxy.serve(&frame);
            assert!(conv.answered(&reply));
        }
    }
}

fn opened(proxy: &mut Proxy) -> Conv {
    let mut conv = Conv::opening(S);
    call(&mut conv, proxy, Fate::Answered);
    assert!(!conv.sending());
    conv
}

#[test]
fn nothing_goes_before_the_reset_is_answered() {
    let mut conv = Conv::opening(S);
    assert!(conv.sending(), "the reset is asked and unanswered");
    conv.take(&[5, 1, 0]).expect("taken");
    assert_eq!(conv.frame(), reset(S), "the reset again, not the greeting");
    assert_eq!(conv.frame(), reset(S));
    assert!(conv.answered(&[0]));
    assert_eq!(conv.frame(), numbered(S, FIRST_SEQ, &[5, 1, 0]), "now the greeting, as number 1");
}

#[test]
fn an_unanswered_frame_is_asked_again_unchanged() {
    let mut proxy = Proxy::default();
    let mut conv = opened(&mut proxy);
    conv.take(b"hello").expect("taken");
    let first = conv.frame();
    conv.take(b"more").expect("taken while the first waits");
    for _ in 0..5 {
        assert_eq!(conv.frame(), first, "same number, same bytes, nothing added");
        assert!(conv.sending());
    }
    assert!(conv.answered(&[0]));
    assert_eq!(conv.frame(), numbered(S, FIRST_SEQ + 1, b"more"), "the next bytes, the next number");
}

#[test]
fn a_poll_moves_the_number_only_when_answered() {
    let mut proxy = Proxy::default();
    let mut conv = opened(&mut proxy);
    assert_eq!(conv.frame(), numbered(S, 1, &[]));
    assert!(!conv.sending(), "a poll is not a send");
    assert_eq!(conv.frame(), numbered(S, 1, &[]), "unanswered: the same number");
    assert!(conv.answered(&[0, 7]));
    assert_eq!(conv.frame(), numbered(S, 2, &[]));
    let mut out = [0u8; 4];
    assert_eq!(conv.read(&mut out), 1);
    assert_eq!(out[0], 7);
}

#[test]
fn a_long_write_goes_in_frames_in_order() {
    let mut proxy = Proxy::default();
    let mut conv = opened(&mut proxy);
    let data: Vec<u8> = (0..CARRY_MAX * 2 + 100).map(|i| i as u8).collect();
    conv.take(&data).expect("taken");
    let mut frames = 0;
    while conv.sending() {
        let f = conv.frame();
        assert!(f.len() <= 9 + CARRY_MAX, "one frame carries at most CARRY_MAX");
        let reply = proxy.serve(&f);
        assert!(conv.answered(&reply));
        frames += 1;
    }
    assert_eq!(frames, 3);
    assert_eq!(proxy.exit, data);
}

#[test]
fn every_byte_reaches_the_exit_once_whatever_the_calls_meet() {
    for seed in 1..400u64 {
        let mut rng = Lcg(seed);
        let mut proxy = Proxy::default();
        let mut conv = Conv::opening(S);
        while conv.sending() {
            call(&mut conv, &mut proxy, rng.fate());
        }
        let mut sent = Vec::new();
        let mut back = Vec::new();
        let mut got = Vec::new();
        for round in 0..40u8 {
            if rng.next() % 3 == 0 {
                let chunk = vec![round; 1 + (rng.next() % 40) as usize];
                conv.take(&chunk).expect("taken");
                sent.extend_from_slice(&chunk);
            }
            if rng.next() % 2 == 0 {
                let reply = vec![round.wrapping_add(100); 1 + (rng.next() % 30) as usize];
                proxy.back.extend_from_slice(&reply);
                back.extend_from_slice(&reply);
            }
            call(&mut conv, &mut proxy, rng.fate());
            let mut buf = [0u8; 64];
            let n = conv.read(&mut buf);
            got.extend_from_slice(&buf[..n]);
        }
        /* Everything settles once calls are answered again. */
        for _ in 0..200 {
            call(&mut conv, &mut proxy, Fate::Answered);
            let mut buf = [0u8; 256];
            let n = conv.read(&mut buf);
            got.extend_from_slice(&buf[..n]);
        }
        assert_eq!(proxy.exit, sent, "seed {seed}: each byte to the exit once, in order");
        assert_eq!(got, back, "seed {seed}: each byte back once, in order");
        assert!(proxy.resets >= 1, "seed {seed}: the reset reached the proxy");
    }
}

#[test]
fn a_close_ends_sending_and_refuses_more() {
    let mut proxy = Proxy::default();
    let mut conv = opened(&mut proxy);
    conv.take(b"GET / HTTP/1.1\r\n\r\n").expect("taken");
    let _ = conv.frame();
    assert!(conv.answered(&[1, b'b', b'y', b'e']), "closed, with the last bytes");
    assert!(!conv.sending() && conv.over());
    assert_eq!(conv.take(b"x"), Err(Refusal::Finished));
    let mut out = [0u8; 8];
    assert_eq!(conv.read(&mut out), 3, "the last bytes are still read");
}

#[test]
fn something_that_is_not_an_answer_ends_the_conversation() {
    let mut proxy = Proxy::default();
    let mut conv = opened(&mut proxy);
    conv.take(b"x").expect("taken");
    let _ = conv.frame();
    assert!(!conv.answered(&[9, 9]), "marker 9 is no answer");
    assert!(conv.over() && !conv.sending());
    let mut fresh = opened(&mut proxy);
    assert!(!fresh.answered(&[]), "nor is nothing at all");
}

#[test]
fn a_refused_call_ends_sending_and_keeps_what_arrived() {
    let mut proxy = Proxy::default();
    let mut conv = opened(&mut proxy);
    proxy.back.extend_from_slice(b"early");
    call(&mut conv, &mut proxy, Fate::Answered);
    conv.take(b"late").expect("taken");
    conv.refused();
    assert!(!conv.sending() && conv.over());
    let mut out = [0u8; 8];
    assert_eq!(conv.read(&mut out), 5);
}

#[test]
fn a_writer_that_never_waits_is_refused_past_the_bound() {
    let mut conv = Conv::opening(S);
    conv.take(&vec![0u8; UNSENT_MAX]).expect("up to the bound");
    assert_eq!(conv.take(&[0]), Err(Refusal::Full));
}

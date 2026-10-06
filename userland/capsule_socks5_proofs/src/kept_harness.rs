// NONOS Operating System (AGPL-3.0-or-later)
/* The run loop's handling of one request, a numbered frame builder and a
 * stand-in for the far end, shared by the kept-answer tests. */

use std::collections::VecDeque;

use crate::kept::{answer, forget};
use crate::who::who;
use crate::reply::{ANSWER_MAX, STREAM_CLOSED, STREAM_OPEN};
use crate::request::{ask, Ask, STREAM_NUMBERED};

pub fn numbered(seq: u32, payload: &[u8]) -> Vec<u8> {
    let mut f = vec![STREAM_NUMBERED];
    f.extend_from_slice(&seq.to_le_bytes());
    f.extend_from_slice(payload);
    f
}

/// The far end as feed() presents it: every body it was handed, and the
/// stream bytes it has to give back, a closed tunnel saying so in its marker.
#[derive(Default)]
pub struct Exit {
    pub carried: Vec<Vec<u8>>,
    pub answers: VecDeque<Vec<u8>>,
    pub closed: bool,
}

impl Exit {
    pub fn with(answers: &[&[u8]]) -> Self {
        Self { answers: answers.iter().map(|a| a.to_vec()).collect(), ..Self::default() }
    }

    /// feed(): carry `body`, then answer with the next bytes, at most `room`
    /// of them, the rest kept for the next call.
    pub fn feed(&mut self, body: &[u8], room: usize) -> Vec<u8> {
        if !body.is_empty() {
            self.carried.push(body.to_vec());
        }
        let mut next = self.answers.pop_front().unwrap_or_default();
        if next.len() > room {
            let tail = next.split_off(room);
            self.answers.push_front(tail);
        }
        let mut out = vec![if self.closed { STREAM_CLOSED } else { STREAM_OPEN }];
        out.extend_from_slice(&next);
        out
    }
}

/// What run.rs does for one request from `pid`.
pub fn serve(pid: u32, frame: &[u8], exit: &mut Exit) -> Vec<u8> {
    match ask(frame) {
        Some(Ask::Numbered(seq, body)) => answer(who(pid, 0), seq, body, |b, room| exit.feed(b, room)),
        Some(Ask::Stream(body)) => exit.feed(body, ANSWER_MAX),
        _ => {
            forget(who(pid, 0));
            Vec::new()
        }
    }
}

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

//! The channel to the core over a scripted pipe: the handshake, frames each
//! way, a ping answered, and down on a refusal, a close or a lost pipe.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::link::core_channel::CoreChannel;
use crate::link::pipe::Pipe;
use crate::link::ws_read::{take, Got};
use crate::link::Transport;

struct Fake(VecDeque<Option<Vec<u8>>>, Rc<RefCell<Vec<u8>>>);

impl Pipe for Fake {
    fn read(&mut self) -> Option<Vec<u8>> {
        self.0.pop_front().unwrap_or(Some(Vec::new()))
    }
    fn write(&mut self, bytes: &[u8]) -> bool {
        self.1.borrow_mut().extend_from_slice(bytes);
        true
    }
}

fn server(op: u8, data: &[u8]) -> Vec<u8> {
    [&[0x80 | op, data.len() as u8][..], data].concat()
}

fn run(comes: Vec<Option<Vec<u8>>>, ticks: usize) -> (CoreChannel<Fake>, Rc<RefCell<Vec<u8>>>) {
    let sent = Rc::new(RefCell::new(Vec::new()));
    let mut c = CoreChannel::new(Fake(comes.into(), sent.clone()), [7; 16]);
    (0..ticks).for_each(|_| c.tick());
    let hello = sent.borrow().windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
    sent.borrow_mut().drain(..hello);
    (c, sent)
}

const OK: &[u8] = b"HTTP/1.1 101 Switching Protocols\r\n\r\n";

#[test]
fn it_is_up_after_101_and_frames_flow_both_ways() {
    let (mut c, sent) =
        run(vec![Some(OK[..10].to_vec()), Some([&OK[10..], &server(1, b"{\"a\":1}")].concat())], 1);
    assert!(!c.up(), "half an answer is not an upgrade");
    c.tick();
    assert!(c.up());
    assert_eq!(c.recv().as_deref(), Some("{\"a\":1}"));
    c.send("{\"cmd\":\"/_get chats 1\"}");
    assert_eq!(take(&mut sent.borrow_mut()), Some(Got::Text("{\"cmd\":\"/_get chats 1\"}".into())));
}

#[test]
fn a_ping_is_answered_and_a_close_a_refusal_or_a_lost_pipe_take_it_down() {
    let (c, sent) = run(vec![Some([OK, &server(9, b"p"), &server(8, b"")].concat())], 1);
    assert_eq!(take(&mut sent.borrow_mut()), Some(Got::Other), "a pong, masked");
    assert!(!c.up());
    assert!(!run(vec![Some(b"HTTP/1.1 403 No\r\n\r\n".to_vec())], 2).0.up());
    assert!(!run(vec![None], 2).0.up());
}

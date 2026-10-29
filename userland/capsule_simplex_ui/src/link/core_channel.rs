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

//! The channel to simplex.core: the WebSocket chat API, carried over one
//! SERVE_ONE connection (`Pipe`). Opened, it asks to upgrade and is up once
//! the core answers 101; then each command goes out as a text frame and each
//! frame that comes in is a response. A ping is answered; a close, a refused
//! upgrade or a pipe that fails leaves it down, and the window shows that.

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

use super::pipe::Pipe;
use super::ws_hello::request;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum State {
    Hello,
    Up,
    Down,
}

pub struct CoreChannel<P: Pipe> {
    pub(super) pipe: P,
    pub(super) state: State,
    pub(super) got: Vec<u8>,
    pub(super) frames: VecDeque<String>,
    mask: u32,
}

impl<P: Pipe> CoreChannel<P> {
    /// Starts the handshake on an open pipe; `seed` keys it and the masks.
    pub fn new(mut pipe: P, seed: [u8; 16]) -> Self {
        let state = match pipe.write(request(&seed).as_bytes()) {
            true => State::Hello,
            false => State::Down,
        };
        let mask = u32::from_le_bytes([seed[0], seed[5], seed[10], seed[15]]) | 1;
        Self { pipe, state, got: Vec::new(), frames: VecDeque::new(), mask }
    }

    pub(super) fn is_up(&self) -> bool {
        self.state == State::Up
    }

    pub(super) fn write_or_down(&mut self, bytes: &[u8]) {
        if !self.pipe.write(bytes) {
            self.state = State::Down;
        }
    }

    pub(super) fn next_mask(&mut self) -> [u8; 4] {
        self.mask ^= self.mask << 13;
        self.mask ^= self.mask >> 17;
        self.mask ^= self.mask << 5;
        self.mask.to_le_bytes()
    }
}

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

//! What the channel to the core does each window tick: read what came, end
//! the handshake, and turn whole frames into responses.

use super::core_channel::{CoreChannel, State};
use super::pipe::Pipe;
use super::ws_hello::{answer, Answer};
use super::ws_read::{take, Got};

impl<P: Pipe> CoreChannel<P> {
    pub(super) fn pump(&mut self) {
        if self.state == State::Down {
            return;
        }
        match self.pipe.read() {
            Some(bytes) => self.got.extend_from_slice(&bytes),
            None => self.state = State::Down,
        }
        if self.state == State::Hello {
            match answer(&self.got) {
                Answer::More => return,
                Answer::Refused => self.state = State::Down,
                Answer::Upgraded(n) => {
                    self.got.drain(..n);
                    self.state = State::Up;
                }
            }
        }
        while self.state == State::Up {
            match take(&mut self.got) {
                Some(Got::Text(t)) => self.frames.push_back(t),
                Some(Got::Ping(d)) => {
                    let pong = super::ws_frame::pong(&d, self.next_mask());
                    self.write_or_down(&pong);
                }
                Some(Got::Closed) => self.state = State::Down,
                Some(Got::Other) => {}
                None => break,
            }
        }
    }
}

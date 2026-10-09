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

//! The two stages that read the caller: its greeting and its request.

extern crate alloc;

use alloc::vec::Vec;

use super::conv::Conv;
use super::rep::reply;
use super::request::connect;
use super::stage::{Stage, Step};
use super::tunnel::Tunnel;
use super::wire::{greeting, method_reply};

impl Conv {
    pub(super) fn greet(&mut self, out: &mut Vec<u8>) -> Step {
        match greeting(&self.unsent) {
            None => Step::Wait,
            Some(Err(())) => Step::Over,
            Some(Ok((no_auth, used))) => {
                self.unsent.drain(..used);
                out.extend_from_slice(&method_reply(no_auth));
                if !no_auth {
                    return Step::Over;
                }
                self.stage = Stage::Request;
                Step::Next
            }
        }
    }

    pub(super) fn request(&mut self, tunnel: &mut impl Tunnel, out: &mut Vec<u8>) -> Step {
        match connect(&self.unsent) {
            None => Step::Wait,
            Some(Err(rep)) => {
                out.extend_from_slice(&reply(rep));
                Step::Over
            }
            Some(Ok((host, port, used))) => {
                self.unsent.drain(..used);
                match tunnel.open(&host, port) {
                    Ok(id) => {
                        self.stage = Stage::Connecting(id);
                        Step::Next
                    }
                    Err(rep) => {
                        out.extend_from_slice(&reply(rep));
                        Step::Over
                    }
                }
            }
        }
    }
}

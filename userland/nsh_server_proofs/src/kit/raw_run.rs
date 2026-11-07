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

//! The raw client against a server: each step sends bytes and gives back
//! what the server answered, opened where keys exist, and its events.

use super::host::{config, Kernel, Keys};
use super::raw::Raw;
use super::rng::Det;
use super::roles::Table;
use super::roles_build::NOW;
use super::run::Host;
use nsh_server::{Event, Server};

pub struct RawRun {
    pub raw: Raw,
    pub server: Host,
    pub events: Vec<Event>,
    pub rng: Det,
    pub now: u64,
}

impl Default for RawRun {
    fn default() -> RawRun {
        RawRun::new()
    }
}

impl RawRun {
    pub fn new() -> RawRun {
        let server =
            Server::new(config(), Kernel::honest(), Keys::default(), Table::shipped(), NOW);
        RawRun {
            raw: Raw::new(),
            server: server.expect("config"),
            events: Vec::new(),
            rng: Det::new(0x77),
            now: NOW,
        }
    }

    /// Feed bytes; the server's answer, unopened.
    pub fn send(&mut self, bytes: &[u8]) -> Vec<u8> {
        let rng = &mut self.rng;
        self.server.feed(bytes, self.now, &mut |b| rng.fill(b));
        self.events.extend(self.server.events());
        self.server.take_output()
    }

    /// Feed bytes; the server's answer, opened.
    pub fn frames(&mut self, bytes: &[u8]) -> Vec<(u8, Vec<u8>)> {
        let out = self.send(bytes);
        self.raw.open(&out)
    }
}

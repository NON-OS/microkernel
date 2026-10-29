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

//! The channel to the core: started as model/start.rs rules, and its
//! endpoint asked for until it answers.

use super::pipe::ServePipe;
use crate::link::core_channel::CoreChannel;
use crate::link::Transport;
use crate::model::start::Start;
use alloc::boxed::Box;
use nonos_libc::mk_uptime_ms;

struct Lazy {
    start: Start,
    open: Option<CoreChannel<ServePipe>>,
}

/// The window's transport: the core, once SERVE_ONE serves it.
pub fn open() -> Box<dyn Transport> {
    Box::new(Lazy { start: Start::new(), open: None })
}

impl Transport for Lazy {
    fn send(&mut self, frame: &str) {
        if let Some(c) = &mut self.open {
            c.send(frame);
        }
    }

    fn recv(&mut self) -> Option<alloc::string::String> {
        self.open.as_mut()?.recv()
    }

    fn tick(&mut self) {
        let now = mk_uptime_ms();
        let step = self.start.tick(now);
        if let Some(word) = step.launch {
            super::launch::launch(&mut self.start, word.as_deref(), now);
        }
        if step.look && self.open.is_none() {
            let seed = (now as u128 ^ 0x5349_4d50_4c45_5800).to_le_bytes();
            self.open = ServePipe::open().map(|p| CoreChannel::new(p, seed));
            self.open.is_some().then(|| self.start.found());
        }
        if let Some(c) = &mut self.open {
            c.tick();
        }
    }

    fn up(&self) -> bool {
        self.open.as_ref().is_some_and(|c| c.up())
    }

    fn label(&self) -> &'static str {
        "simplex.core"
    }

    fn start(&mut self) -> Option<&mut Start> {
        Some(&mut self.start)
    }
}

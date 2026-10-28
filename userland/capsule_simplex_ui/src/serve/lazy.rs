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

//! A channel to the core that is not open yet: each tick it asks once more,
//! every quarter second, for thirty seconds from the window's start.

use super::pipe::ServePipe;
use crate::link::core_channel::CoreChannel;
use crate::link::Transport;
use alloc::boxed::Box;
use nonos_libc::{crypto_random, mk_app_launch, mk_uptime_ms};

/// The core's listing, which MkAppLaunch starts in its own box.
const CORE: &[u8] = b"linux.kali.nonos-simplex-chat";
const TRY_MS: i64 = 30_000;
const EVERY_MS: i64 = 250;

struct Lazy {
    since: i64,
    last: i64,
    open: Option<CoreChannel<ServePipe>>,
}

/// The window's transport: the core, once SERVE_ONE serves it.
pub fn open() -> Box<dyn Transport> {
    let _ = mk_app_launch(CORE);
    Box::new(Lazy { since: mk_uptime_ms(), last: i64::MIN, open: None })
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
        if self.open.is_none() && now - self.since < TRY_MS && now - self.last >= EVERY_MS {
            self.last = now;
            let mut seed = [0u8; 16];
            if crypto_random(seed.as_mut_ptr(), seed.len()) >= 0 {
                self.open = ServePipe::open().map(|p| CoreChannel::new(p, seed));
            }
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
}

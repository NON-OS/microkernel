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

//! A scripted stream through a proxy, for the routed reader's tests.

use crate::wallet::net::routed_read::{Bounds, Source};

/// A stream through a proxy, scripted: each read either brings `bytes`
/// after `after_ms`, or nothing until the wait runs out; `None` in the
/// script is the far end finishing.
pub(super) struct Script {
    pub(super) now: i64,
    pub(super) steps: Vec<Option<(i64, Vec<u8>)>>,
    pub(super) waits: Vec<u64>,
    pub(super) broke: bool,
}

impl Script {
    pub(super) fn new(steps: Vec<Option<(i64, Vec<u8>)>>) -> Script {
        Script { now: 0, steps, waits: Vec::new(), broke: false }
    }
}

impl Source for Script {
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        self.waits.push(wait_ms);
        if self.broke {
            return Err("broke");
        }
        if self.steps.is_empty() {
            self.now += wait_ms as i64;
            return Ok(0);
        }
        match self.steps.remove(0) {
            None => Ok(0),
            Some((after, bytes)) if after <= wait_ms as i64 => {
                self.now += after;
                let n = bytes.len().min(into.len());
                into[..n].copy_from_slice(&bytes[..n]);
                Ok(n)
            }
            Some(later) => {
                self.now += wait_ms as i64;
                self.steps.insert(0, Some((later.0 - wait_ms as i64, later.1)));
                Ok(0)
            }
        }
    }

    fn now_ms(&self) -> i64 {
        self.now
    }
}

pub(super) const BOUNDS: Bounds =
    Bounds { first_ms: 60_000, quiet_ms: 8_000, total_ms: 90_000, max: 64 };

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

/*
 * One line per file, rewritten as it comes: how much of how much, how
 * fast, and the time left once enough has come to say it honestly
 * (`path::eta`). The same numbers, for the whole tier, are what the store
 * is answered (`serve`).
 */

use alloc::format;
use alloc::string::String;

use nonos_libc::mk_uptime_ms;

use crate::out::{over, say, size};
use super::eta::eta;
use crate::path::duration;
use crate::serve;
use crate::status_wire::{Status, FETCHING};

/* A step of the line at least this far apart, and at most a hundred a file. */
const STEP: u64 = 8 << 20;

/* The tier a file is part of: its bytes in all, those before this file, and the route's code. */
#[derive(Clone, Copy)]
pub struct Whole {
    pub total: u64,
    pub before: u64,
    pub route: u8,
}

pub struct Progress<'a> {
    name: &'a str,
    bytes: u64,
    shown: u64,
    from: u64,
    since_ms: i64,
    whole: Whole,
}

impl<'a> Progress<'a> {
    pub fn new(name: &'a str, bytes: u64, from: u64, whole: Whole) -> Self {
        let p = Progress { name, bytes, shown: from, from, since_ms: mk_uptime_ms(), whole };
        if from > 0 {
            let kept = format!("  {name}: going on from {} of {}, kept", size(from), size(bytes));
            say(&format!("{kept} from before"));
        }
        p.publish(from, 0);
        p
    }

    pub fn at(&mut self, at: u64) {
        serve::answer();
        if at - self.shown < STEP.max(self.bytes / 100) && at != self.bytes {
            return;
        }
        self.shown = at;
        let ms = (mk_uptime_ms() - self.since_ms).max(1) as u64;
        let got = at - self.from;
        let rate = got * 1000 / ms;
        let left = eta(self.whole.total - self.whole.before - at, got, ms);
        /* A rate is told the store only once the time left can be said from it. */
        self.publish(at, if left.is_some() { rate } else { 0 });
        let pct = at * 100 / self.bytes.max(1);
        let tail = left.map_or(String::new(), |s| format!(", {} left", duration(s)));
        over(&format!(
            "  {}: {pct}%, {} of {}, {}/s{tail}",
            self.name,
            size(at),
            size(self.bytes),
            size(rate)
        ));
    }

    /* What the store is answered: the tier's bytes on the volume, of all of them. */
    fn publish(&self, at: u64, rate: u64) {
        serve::set(Status {
            stage: FETCHING,
            route: self.whole.route,
            total: self.whole.total,
            done: self.whole.before + at,
            rate,
            try_n: 0,
            tries: 0,
        });
    }

    /* A line of its own under the progress, which goes on below it. */
    pub fn note(&self, text: &str) {
        over(text);
        say("");
    }

    pub fn end(&self) {
        say("");
    }
}

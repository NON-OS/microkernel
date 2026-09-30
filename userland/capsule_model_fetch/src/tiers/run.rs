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
 * The table `qwen tiers` prints, and a mark on each tier this machine does
 * not have the memory to run.
 */

use alloc::format;
use alloc::string::String;

use super::memory::memory;
use super::status::{status, Status};
use crate::catalogue::Catalogue;
use crate::errno::said;
use crate::net::Route;
use crate::out::{say, size};
use crate::pins::all;

pub fn run(cat: &Catalogue) -> i32 {
    let from = if cat.base.is_empty() { "the upstream files" } else { cat.base.as_str() };
    say(&format!(
        "Qwen tiers (catalogue {}, from {from}, {}):",
        cat.serial,
        Route::chosen().name()
    ));
    let machine = memory();
    say(&format!("  {:<14} {:>9} {:>9}  {}", "tier", "download", "memory", "on this machine"));
    for tier in &cat.tiers {
        let mut line = match status(tier) {
            Status::Installed => String::from("installed"),
            Status::Partial(have) => {
                format!("{}% fetched, `qwen get` goes on", have * 100 / tier.bytes())
            }
            Status::Available => String::from("available"),
            Status::NoRoom => String::from("available; no room on the data volume"),
            Status::Unkept => String::from("not fetched: its file names are too long to keep"),
            Status::Unknown(e) => format!("unknown: {}", said(e)),
        };
        if let Some(m) = machine.filter(|&m| tier.memory > m) {
            line.push_str(&format!("; too large to run here ({} of memory)", size(m)));
        }
        let (word, bytes, need) = (&tier.word, size(tier.bytes()), size(tier.memory));
        say(&format!("  {word:<14} {bytes:>9} {need:>9}  {line}"));
    }
    let mut missing: alloc::vec::Vec<&str> = all().map(|p| p.tier).collect();
    missing.dedup();
    missing.retain(|w| cat.tier(w.as_bytes()).is_none());
    if !missing.is_empty() {
        say(&format!("  not in this catalogue: {}", missing.join(" ")));
    }
    say("  `qwen get TIER...` fetches tiers; Ctrl-C stops, and it goes on next time");
    0
}

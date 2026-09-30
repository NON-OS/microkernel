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

/* `qwen get TIER...`: the tiers named, each file in turn, and what became of
 * each, said at the Terminal. */

use alloc::format;
use alloc::vec::Vec;

use super::file::{fetch, Got};
use crate::catalogue::{Catalogue, Tier};
use crate::net::Route;
use crate::out::{say, size};
use crate::tiers::memory;

pub fn run(cat: &Catalogue, words: &[Vec<u8>]) -> i32 {
    let mut chosen: Vec<&Tier> = Vec::new();
    for word in words {
        match cat.tier(word) {
            Some(t) if !chosen.iter().any(|c| c.word == t.word) => chosen.push(t),
            Some(_) => {}
            None => {
                let w = core::str::from_utf8(word).unwrap_or("?");
                say(&format!("qwen get: {w} is not a tier; `qwen tiers` lists them"));
                return 2;
            }
        }
    }
    let (route, machine) = (Route::chosen(), memory());
    let mut refused = 0;
    for tier in chosen {
        let files = tier.files.len();
        say(&format!(
            "qwen get {}: {} in {files} file(s), {} of memory to run, {}",
            tier.word,
            size(tier.bytes()),
            size(tier.memory),
            route.name()
        ));
        if let Some(m) = machine.filter(|&m| tier.memory > m) {
            say(&format!("  this machine has {} of memory: it can keep it, not run it", size(m)));
        }
        let mut whole = true;
        for f in &tier.files {
            match fetch(route, f) {
                Ok(Got::Already) => say(&format!("  {}: already here, verified", f.name)),
                Ok(Got::Fetched) => say(&format!("  {}: verified against the signed pin", f.name)),
                Err(why) => {
                    say(&format!("  {}: refused: {why}", f.name));
                    whole = false;
                    break;
                }
            }
        }
        refused += usize::from(!whole);
        if whole {
            say(&format!("qwen get {}: done; start it with `qwen {}`", tier.word, tier.word));
        }
    }
    i32::from(refused > 0)
}

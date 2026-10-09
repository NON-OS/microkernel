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

/* `qwen get [--direct] TIER...`: the tiers named, each file in turn, and
 * what became of each, said at the Terminal. The path a download takes is
 * said first and is the chosen network's unless the person asked for a
 * direct one (`path`). The exit status is the first refusal's reason
 * (`crate::exit`), which the installer reads when it is the one asking. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::file::{fetch, Got};
use super::memory_need::too_large;
use super::progress::Whole;
use crate::catalogue::{Catalogue, File, Tier};
use crate::errno::no_room;
use crate::exit::{DONE, NO_ROOM, TOO_LITTLE_MEMORY, USAGE};
use crate::need::{Room, STICK_TIER};
use crate::net::Route;
use crate::out::{say, size};
use super::anyone::ready;
use super::offer::{offer_line, route_for, route_said};
use crate::status_wire::{ANYONE, DIRECT, NYM};
use crate::tiers::{memory, room, status, Status};

/* The word that asks for a direct download, for this run only. */
pub const DIRECT_WORD: &[u8] = b"--direct";

/*
 * The stick tier is carried by release sticks, and opening it there imports
 * it with no network; the fetcher cannot read the stick, so it says so and
 * downloads what it was asked for.
 */
const STICK_NOTE: &str = "  the stick tier: a release stick carries it, and opening it there (`qwen \
     qwen3-0.6b`, or Install in the Store) imports it with no network; this downloads it";

const DIRECT_NOTE: &str = "  over a direct connection, as asked for this download: faster, and \
     the mirror sees this machine's address";

pub fn run(cat: &Catalogue, words: &[Vec<u8>]) -> i32 {
    let direct = words.iter().any(|w| w == DIRECT_WORD);
    let mut chosen: Vec<&Tier> = Vec::new();
    for word in words.iter().filter(|w| *w != DIRECT_WORD) {
        match cat.tier(word) {
            Some(t) if !chosen.iter().any(|c| c.word == t.word) => chosen.push(t),
            Some(_) => {}
            None => {
                let w = core::str::from_utf8(word).unwrap_or("?");
                say(&format!("qwen get: {w} is not a tier; `qwen tiers` lists them"));
                return USAGE;
            }
        }
    }
    if chosen.is_empty() {
        say("qwen get: name one or more tiers; `qwen tiers` lists them");
        return USAGE;
    }
    let (planned, machine, room) = (route_for(Route::for_installs(), direct), memory(), room());
    let mut status = DONE;
    for tier in chosen {
        let left = tier.bytes() - have(tier);
        /* Installs download over Anyone; one that is not up yet is waited for. */
        let shown = if direct { planned } else { Route::Anon(0) };
        say(&format!(
            "qwen get {}: {} in {} file(s), {} still to come, {} of memory to run, {}",
            tier.word,
            size(tier.bytes()),
            tier.files.len(),
            size(left),
            size(tier.memory),
            route_said(shown)
        ));
        if let Some(why) = too_large(&tier.word, tier.bytes(), tier.memory, machine, room) {
            say(&format!("qwen get {}: refused: {why}", tier.word));
            if status == DONE {
                status = TOO_LITTLE_MEMORY;
            }
            continue;
        }
        if tier.word == STICK_TIER {
            say(STICK_NOTE);
        }
        let route = match (direct, left) {
            (true, _) | (false, 0) => planned,
            (false, _) => match ready(tier.bytes(), tier.bytes() - left) {
                Ok(route) => route,
                Err(why) => {
                    say(&format!("qwen get {}: refused: {}", tier.word, why.said));
                    if status == DONE {
                        status = why.status;
                    }
                    continue;
                }
            },
        };
        if direct {
            say(DIRECT_NOTE);
        } else if let Some(offer) = offer_line(route, left, &tier.word) {
            say(&format!("  {offer}"));
        }
        let got = files(tier, route, room);
        match got {
            Ok(()) => {
                say(&format!("qwen get {}: done; start it with `qwen {}`", tier.word, tier.word))
            }
            Err(why) if status == DONE => status = why,
            Err(_) => {}
        }
    }
    status
}

/* Every file of `tier` in turn; the first refusal's status. */
fn files(tier: &Tier, route: Route, room: Room) -> Result<(), i32> {
    let mut before = 0;
    for f in &tier.files {
        let whole = Whole { total: tier.bytes(), before, route: code(route) };
        match fetch(route, f, whole) {
            Ok(Got::Already) => say(&format!("  {}: already here, verified", f.name)),
            Ok(Got::Fetched) => say(&format!(
                "  {}: SHA-256 {} is the signed pin's: verified, kept",
                f.name,
                short(f)
            )),
            Err(why) => {
                let said = match why.status {
                    NO_ROOM => no_room(room == Room::Memory, tier.bytes() - before),
                    _ => why.said,
                };
                say(&format!("  {}: refused: {said}", f.name));
                return Err(why.status);
            }
        }
        before += f.bytes;
    }
    Ok(())
}

/* Bytes of `tier` the volume holds already, whole files and kept parts. */
fn have(tier: &Tier) -> u64 {
    match status(tier) {
        Status::Installed => tier.bytes(),
        Status::Partial(have) => have,
        _ => 0,
    }
}

/* The route as the store is told it. */
fn code(route: Route) -> u8 {
    match route {
        Route::Nym(_) => NYM,
        Route::Anon(_) => ANYONE,
        Route::Direct => DIRECT,
        Route::Down(_) => 0,
    }
}

/* The first eight bytes of a file's pinned SHA-256, in hex. */
fn short(f: &File) -> String {
    f.sha256[..8].iter().map(|b| format!("{b:02x}")).collect()
}

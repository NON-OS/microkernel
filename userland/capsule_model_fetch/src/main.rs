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
 * model-fetch: `qwen get` and `qwen tiers` in the Terminal. The tiers are
 * those of a catalogue signed with the marketplace operator key and built
 * in, each entry equal to a pin from the Linux personality's own tables; a
 * build without the key has none. A file comes from the NONOS model
 * repository when the build named one, else from Hugging Face, over TLS
 * through the machine's network, and is fed to the kernel chunk by chunk,
 * which links it only when its SHA-256 is the pin; a cut download goes on
 * from the kernel's mark. It never sees a conversation.
 */

#![no_std]
#![no_main]

extern crate alloc;

mod args;
mod caps;
mod catalogue;
mod errno;
mod exit;
mod feed;
mod get;
mod http;
mod need;
mod net;
mod out;
mod path;
mod pins;
mod remove;
mod serve;
mod size;
mod status_wire;
mod status_write;
mod tiers;

use nonos_libc::{heap_init, mk_exit};

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    mk_exit(run(&args::words()))
}

/*
 * `get TIER...`, `remove TIER...` or `tiers`, as the Terminal passes them,
 * or `get TIER` and `remove TIER` as the Linux installer does for a tier
 * installed or uninstalled from the store; the exit status, one of `exit`'s.
 */
fn run(words: &[alloc::vec::Vec<u8>]) -> i32 {
    /*
     * `remove TIER...` reads the pins, not the catalogue: what is on the
     * volume can be taken off whatever catalogue, if any, this build has.
     */
    if let Some((verb, tiers)) = words.split_first() {
        if verb == b"remove" && !tiers.is_empty() {
            return remove::run(tiers);
        }
    }
    let cat = match catalogue::load() {
        Ok(cat) => cat,
        Err(why) => {
            out::say(&alloc::format!("qwen: {why}"));
            return exit::NO_CATALOGUE;
        }
    };
    match words.split_first() {
        Some((verb, tiers)) if verb == b"get" && !tiers.is_empty() => get::run(&cat, tiers),
        Some((verb, [])) if verb == b"tiers" => tiers::run(&cat),
        _ => {
            out::say("model-fetch: usage: get TIER...  |  remove TIER...  |  tiers");
            exit::USAGE
        }
    }
}

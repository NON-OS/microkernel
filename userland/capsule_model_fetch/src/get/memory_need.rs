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
 * Whether a tier is worth downloading on this machine. The signed catalogue
 * says what each tier needs in memory, worked out as the chat program works
 * it out before it loads (tools/nonos_qwen_tier/shapes.py): weights, the
 * KV cache of the fewest positions it starts with, and a margin. Whether
 * that fits is need.rs's rule (`fits`), the one setup's Qwen step and the
 * store's cards hold a tier to: on an installed NONOS the run must leave
 * 1 GiB for the system; on a live boot the file is held in memory too,
 * beside the run, under what the kernel keeps free. A tier that does not
 * fit could never run here, so gigabytes are not spent on it. Memory the
 * kernel would not report refuses nothing: the chat program checks again,
 * against what is free, before it loads. Pure, so model_fetch_proofs holds
 * the rule.
 */

use alloc::format;
use alloc::string::String;

use crate::need::{fits, session_keep, Room, SYSTEM};
use crate::size::size;

/*
 * The sentence a tier of `weights` bytes of files, needing `need` to run,
 * is refused with in `room`, if it is.
 */
pub fn too_large(
    word: &str,
    weights: u64,
    need: u64,
    machine: Option<u64>,
    room: Room,
) -> Option<String> {
    let m = machine.filter(|&m| !fits(room, weights, need, m))?;
    let (held, kept) = match room {
        Room::Disk => (String::new(), SYSTEM),
        Room::Memory => (
            format!(", and on this live boot its {} file is held in memory too", size(weights)),
            session_keep(m),
        ),
    };
    Some(format!(
        "{word} needs {} of memory to run{held}; this machine has {} in all, {} of it kept for \
         the system, so it was not downloaded; `qwen tiers` shows which tiers fit",
        size(need),
        size(m),
        size(kept)
    ))
}

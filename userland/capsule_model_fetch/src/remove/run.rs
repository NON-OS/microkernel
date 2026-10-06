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
 * `remove TIER...`: take each tier's model files off the data volume, with
 * the record the kernel kept beside each. StreamImport, the right this
 * fetcher holds to bring a model in, is the right the kernel asks to take
 * one out. The tiers are the pins' own, compiled in, so a tier fetched
 * under an older catalogue, or brought in from the disk plan, is removed
 * just the same, and a build without a catalogue can still remove.
 *
 * A file already gone is as good as removed. The kernel refuses a name a
 * stream is still being fed to (EBUSY), a download under way; a download
 * paused part way goes with its file, its mark and its record.
 */

use alloc::format;
use alloc::vec::Vec;

use nonos_libc::data::mk_data_remove;

use super::plan::plan;
use crate::errno::said;
use crate::exit::{of_errno, BUSY, DONE, USAGE};
use crate::feed::{begin, Start};
use crate::out::say;
use crate::pins::all;

const ENOENT: i64 = -2;
const EBUSY: i64 = -16;

pub fn run(words: &[Vec<u8>]) -> i32 {
    let pins: Vec<(&str, &[u8])> = all().map(|p| (p.tier, p.name)).collect();
    let mut status = DONE;
    for word in words {
        let tier = core::str::from_utf8(word).unwrap_or("");
        if !pins.iter().any(|(t, _)| *t == tier) {
            say(&format!("qwen remove: {tier} is not a tier; `qwen tiers` lists them"));
            return USAGE;
        }
        let files = plan(tier, &pins, whole);
        let mut kept = 0usize;
        for name in &files {
            let file = core::str::from_utf8(name.strip_prefix(b"/").unwrap_or(name)).unwrap_or("?");
            match mk_data_remove(name) {
                0 => say(&format!("  {file}: removed")),
                ENOENT => say(&format!("  {file}: not on this machine")),
                rc => {
                    kept += 1;
                    say(&format!("  {file}: kept: {}", said(rc)));
                    if status == DONE {
                        status = if rc == EBUSY { BUSY } else { of_errno(rc) };
                    }
                }
            }
        }
        match kept {
            0 => say(&format!("qwen remove {tier}: done")),
            n => say(&format!("qwen remove {tier}: {n} file(s) kept; the reasons are above")),
        }
    }
    status
}

/* Every file of `tier` on the volume, verified: asked without holding a stream. */
fn whole(tier: &str) -> bool {
    let mut files = all().filter(|p| p.tier == tier).peekable();
    files.peek().is_some()
        && files.all(|p| matches!(begin(p.name, &p.sha256, p.bytes, true), Ok(Start::Done)))
}

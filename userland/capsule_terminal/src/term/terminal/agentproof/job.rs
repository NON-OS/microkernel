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

//! The job a headless AgentProof run carries out, as the image holds it:
//!
//! ```text
//! nonos-agentproof-job 1
//! box <box word>
//! run <program> [arguments]
//! ```
//!
//! It becomes one Terminal line, `linux --box <box> <program> [arguments]`.
//! Every word is from a small set of characters with no quote, redirect,
//! pipe or separator, so the line means exactly what the job says and the
//! Terminal's own parser has nothing to interpret.

use alloc::vec::Vec;

/// Where the image holds the job (the Linux tree's /usr/share/agentproof).
pub const JOB_PATH: &[u8] = b"/linux/usr/share/agentproof/job";
/// The largest job file read.
pub const JOB_MAX: usize = 1024;
/// The Terminal keeps eight words a line; `linux --box <box>` takes three.
pub const RUN_WORDS: usize = 5;

fn word_ok(w: &[u8]) -> bool {
    !w.is_empty() && w.iter().all(|b| b.is_ascii_alphanumeric() || b"/._-=:,+@%".contains(b))
}

/// The Terminal line the job means, or why the job is refused.
pub fn line_of(text: &[u8]) -> Result<Vec<u8>, &'static str> {
    if text.len() > JOB_MAX {
        return Err("a job is at most 1024 bytes");
    }
    let mut lines = text.split(|b| *b == b'\n').filter(|l| !l.is_empty());
    if lines.next() != Some(b"nonos-agentproof-job 1".as_slice()) {
        return Err("the first line is `nonos-agentproof-job 1`");
    }
    let boxed = lines.next().and_then(|l| l.strip_prefix(b"box ")).ok_or("no `box` line")?;
    let run = lines.next().and_then(|l| l.strip_prefix(b"run ")).ok_or("no `run` line")?;
    if lines.next().is_some() {
        return Err("a job has three lines");
    }
    let words: Vec<&[u8]> = run.split(|b| *b == b' ').collect();
    if !word_ok(boxed) || !words.iter().all(|w| word_ok(w)) {
        return Err("a word holds only letters, digits and / . _ - = : , + @ %");
    }
    if words.len() > RUN_WORDS {
        return Err("`run` is at most five words");
    }
    let mut line = b"linux --box ".to_vec();
    line.extend_from_slice(boxed);
    for w in words {
        line.push(b' ');
        line.extend_from_slice(w);
    }
    Ok(line)
}

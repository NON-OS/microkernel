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

//! A run request's words, read without the kernel: which package, and
//! whether it runs in its own window or on the terminal that started it.
//! Pure, so the host proofs hold the parser and the tier arguments to it.

use alloc::string::String;
use alloc::vec::Vec;

/// Where a run's program talks to the person: a fixed set, nothing else.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Its own window, as the store starts it. The default.
    Window,
    /// The terminal that started it: stdin, stdout and stderr are that
    /// terminal's, and nothing the program prints reaches a log.
    Cli,
}

/// `run\0<name>`, or `run\0<name>\0cli` for a terminal run. Any other
/// third word leaves the window, so only the exact word asks for a terminal.
pub fn parse(args: &[u8]) -> Option<(String, Mode)> {
    let mut parts = args.split(|b| *b == 0);
    if parts.next()? != b"run" {
        return None;
    }
    let name = parts.next().filter(|s| !s.is_empty())?;
    let mode = match parts.next() {
        Some(b"cli") => Mode::Cli,
        _ => Mode::Window,
    };
    Some((String::from(core::str::from_utf8(name).ok()?), mode))
}

impl Mode {
    /// A shipped tier's arguments for this mode. On a terminal the chat
    /// runs without its window: "-ui" and the word after it are left out,
    /// and the rest, "-m <model>" among them, is kept as it is.
    pub fn tier_args(self, tier: &[&[u8]]) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        let mut words = tier.iter();
        while let Some(word) = words.next() {
            if self == Mode::Cli && *word == b"-ui" {
                let _ = words.next();
                continue;
            }
            out.push(word.to_vec());
        }
        out
    }
}

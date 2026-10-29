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

//! The one word a served launch gave this run (the kernel's
//! app_launch_word.rs), read from `run\0<name>\0word\0<word>` and held again
//! to the kernel's bounds before it reaches the program, as its last
//! argument after its listed ones. Pure, so the host proofs hold it.

/// The longest word, as the kernel bounds it.
pub const MOST: usize = 96;

/// The word, when the request carries one that is plain UTF-8 within MOST.
pub fn word(args: &[u8]) -> Option<&[u8]> {
    let mut parts = args.split(|b| *b == 0);
    let (run, _name, mark, word) = (parts.next()?, parts.next()?, parts.next()?, parts.next()?);
    let text = core::str::from_utf8(word).ok()?;
    let plain = !text.is_empty() && !text.bytes().any(|b| b < 0x20 || b == 0x7f);
    (run == b"run" && mark == b"word" && word.len() <= MOST && plain).then_some(word)
}

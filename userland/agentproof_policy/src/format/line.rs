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

//! One line of a policy file, taken into what the file has said so far.

use alloc::vec::Vec;

use super::words::{base, write};
use super::Base;
use crate::boxed::terminal_run::NO_NETWORK;

pub use super::words::MAX_WRITES;

#[derive(Default)]
pub struct Seen {
    pub header: bool,
    pub base: Option<Base>,
    pub network: bool,
    /// Each folder with the line that named it.
    pub writes: Vec<(usize, Vec<u8>)>,
    pub line: usize,
}

pub fn take(seen: &mut Seen, raw: &[u8]) -> Result<(), &'static str> {
    seen.line += 1;
    if !raw.iter().all(|b| *b == b' ' || b.is_ascii_graphic()) {
        return Err("a line is printable ASCII and spaces, with no tab or carriage return");
    }
    let words: Vec<&[u8]> = raw.split(|b| *b == b' ').filter(|w| !w.is_empty()).collect();
    let Some((key, args)) = words.split_first() else { return Ok(()) };
    if key.starts_with(b"#") {
        return Ok(());
    }
    if !seen.header {
        return match (*key, args) {
            (b"nonos-box", [b"1"]) => {
                seen.header = true;
                Ok(())
            }
            (b"nonos-box", _) => Err("this reader knows `nonos-box 1` only"),
            _ => Err("the first line is `nonos-box 1`"),
        };
    }
    let [arg] = args else { return Err("a line is its name and one word") };
    match *key {
        b"base" if seen.base.is_some() => Err("base is named once"),
        b"base" => {
            seen.base = Some(base(arg)?);
            Ok(())
        }
        b"network" if seen.network => Err("network is named once"),
        b"network" if *arg == b"none" => {
            seen.network = true;
            Ok(())
        }
        b"network" | b"connect" => Err(NO_NETWORK),
        b"write" => write(seen, arg),
        b"nonos-box" => Err("the header is named once"),
        _ => Err("a line is base, write, network or a # comment"),
    }
}

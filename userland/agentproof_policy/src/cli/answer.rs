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

//! The answer to a verb on a file's bytes, and to words it cannot use.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::{Answer, USAGE};
use crate::explain::{explain, hex};
use crate::{compile, parse};

pub fn answer(verb: &str, file: &str, text: &[u8]) -> Answer {
    let compiled = parse(text)
        .map_err(|e| match e.line {
            0 => format!("{file}: {}", e.why),
            n => format!("{file}:{n}: {}", e.why),
        })
        .and_then(|p| compile(&p).map_err(|why| format!("{file}: {why}")));
    match compiled {
        Err(why) => Answer {
            code: 1,
            out: Vec::new(),
            err: [format!("agentproof: refused: {why}")].to_vec(),
        },
        Ok(c) => {
            let out = match verb {
                "word" => [String::from_utf8_lossy(&c.word).into_owned()].to_vec(),
                "hash" => [hex(&c.hash)].to_vec(),
                _ => explain(&c),
            };
            Answer { code: 0, out, err: Vec::new() }
        }
    }
}

pub fn unreadable(file: &str, why: &str) -> Answer {
    Answer { code: 2, out: Vec::new(), err: [format!("agentproof: {file}: {why}")].to_vec() }
}

pub fn usage() -> Answer {
    Answer { code: 2, out: Vec::new(), err: [USAGE.to_string()].to_vec() }
}

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
//! What `agentproof-receipt verify` answers, as a pure function of the two
//! files' bytes. Exit 0: verified; 1: refused, by name; 2: unusable input.

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::{trust_file, verify};

pub const USAGE: &str = "usage: agentproof-receipt verify [--release] <receipt> <trust>
       agentproof-receipt pin <receipt>
  verify checks a NAPR v0 receipt offline against a nonos-receipt-trust 1 file and says
  whether it is from a release; --release refuses one that is not. pin writes the file
  it verifies under, pinned on first use (docs/agentproof/receipt-verify.md)";

pub struct Answer {
    pub code: u8,
    pub out: Vec<String>,
    pub err: Vec<String>,
}

pub fn answer(receipt: &[u8], trust: &str) -> Answer {
    answer_in(receipt, trust, false)
}

/// With `release`, a receipt that verified but is not from a release exits 1.
pub fn answer_in(receipt: &[u8], trust: &str, release: bool) -> Answer {
    let trust = match trust_file::read(trust) {
        Ok(t) => t,
        Err((line, why)) => return fail(2, format!("trust file line {line}: {why}")),
    };
    let v = match verify(receipt, &trust) {
        Ok(v) => v,
        Err(r) => return fail(1, format!("refused: {} ({r:?})", r.name())),
    };
    let out = crate::cli_says::lines(&v);
    match (release, &v.is_release) {
        (true, verify::Release::NotRelease(_)) => Answer {
            code: 1,
            out,
            err: vec![format!("agentproof-receipt: {}", v.is_release.said())],
        },
        _ => Answer { code: 0, out, err: vec![] },
    }
}

fn fail(code: u8, why: String) -> Answer {
    Answer { code, out: vec![], err: vec![format!("agentproof-receipt: {why}")] }
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

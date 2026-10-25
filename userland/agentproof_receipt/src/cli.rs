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

pub const USAGE: &str = "usage: agentproof-receipt verify <receipt> <trust>
  checks a NAPR v0 receipt offline against a nonos-receipt-trust 1 file
  (docs/agentproof/receipt.md)";

pub struct Answer {
    pub code: u8,
    pub out: Vec<String>,
    pub err: Vec<String>,
}

pub fn answer(receipt: &[u8], trust: &str) -> Answer {
    let trust = match trust_file::read(trust) {
        Ok(t) => t,
        Err((line, why)) => return fail(2, format!("trust file line {line}: {why}")),
    };
    let v = match verify(receipt, &trust) {
        Ok(v) => v,
        Err(r) => return fail(1, format!("refused: {} ({r:?})", r.name())),
    };
    let mut out =
        vec![format!("receipt  verified: NAPR v0; the AK is the one pinned, with no EK chain")];
    out.push(format!("release  {} pcr9 {}", v.release, hex(&v.pcr9)));
    match v.program {
        Some(p) => out.push(format!("program  {} app {}", hex(&p), hex(&v.app_id))),
        None => out.push("program  the personality's own BusyBox, which names no program".into()),
    }
    out.push(format!("box      {} {}", v.box_word, hex(&v.box_hash)));
    if !v.enforced {
        out.push("mode     LEARN, not enforced: this run was held to no policy of yours".into());
    }
    out.push(format!("sealed   by {} as family {:016x}", hex(&v.supervisor), v.seal_no));
    out.push("counts   not carried in v0".into());
    Answer { code: 0, out, err: vec![] }
}

fn fail(code: u8, why: String) -> Answer {
    Answer { code, out: vec![], err: vec![format!("agentproof-receipt: {why}")] }
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

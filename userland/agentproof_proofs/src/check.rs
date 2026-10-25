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

//! What any bytes handed to the policy reader must leave: a refusal with a
//! line inside the file, or a policy whose word capsule_linux reads back as
//! the same box, with the same hash, from a file written out again.

use agentproof_policy::boxed::hash::box_hash;
use agentproof_policy::boxed::preset::parse as word_parse;
use agentproof_policy::compile::{word, MAX_WORD};
use agentproof_policy::format::{Base, Policy, MAX_FILE, MAX_WRITES};
use agentproof_policy::{compile, parse, Compiled};

/// The policy written out again as a file, in its one plain spelling.
pub fn spell(p: &Policy) -> Vec<u8> {
    let base: &[u8] = match p.base {
        Base::Strict => b"strict",
        Base::Offline => b"offline",
        Base::Folders => b"folders",
    };
    let mut out = b"nonos-box 1\nbase ".to_vec();
    out.extend_from_slice(base);
    out.extend_from_slice(b"\nnetwork none\n");
    for w in &p.writes {
        out.extend_from_slice(b"write ");
        out.extend_from_slice(w);
        out.push(b'\n');
    }
    out
}

pub fn check_file(text: &[u8]) -> Option<Compiled> {
    let lines = text.split(|b| *b == b'\n').count();
    let policy = match parse(text) {
        Err(e) => {
            assert!(e.line <= lines, "a refusal names a line the file has");
            assert!(!e.why.is_empty());
            return None;
        }
        Ok(p) => p,
    };
    assert!(text.len() <= MAX_FILE);
    assert!(policy.writes.len() <= MAX_WRITES);
    assert_eq!(policy.writes.is_empty(), policy.base != Base::Folders);
    let spelt = spell(&policy);
    if spelt.len() <= MAX_FILE {
        assert_eq!(parse(&spelt).as_ref(), Ok(&policy), "it reads back as written");
    }
    let c = compile(&policy).ok()?;
    assert!(c.word.len() <= MAX_WORD);
    assert_eq!(c.word, word(&policy));
    let again = word_parse(&c.word).expect("capsule_linux takes the word");
    assert_eq!(again, c.policy);
    assert_eq!(box_hash(&again), c.hash);
    assert!(again.dests.is_empty(), "a terminal box has no network");
    Some(c)
}

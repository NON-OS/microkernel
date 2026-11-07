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


//! Reads the generated Lean data back: the capability bits, the ABI list and
//! the cap table as `Req` terms, so a test can evaluate the model's table on
//! the same token the kernel's table sees.

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

pub(super) const CAP_BITS: &str = include_str!("../../../../formal/Formal/Data/CapBits.lean");
pub(super) const SYSCALLS: &str = include_str!("../../../../formal/Formal/Data/Syscalls.lean");

/// `Formal.Req`.
#[derive(Debug)]
pub(super) enum Req {
    Yes,
    Valid,
    Launcher,
    Has(u32),
    Not(Box<Req>),
    And(Box<Req>, Box<Req>),
    Or(Box<Req>, Box<Req>),
}

impl Req {
    /// `Formal.Req.eval`: `valid` is a non-empty word, as for a token
    /// without expiry.
    pub(super) fn eval(&self, perms: u64, launcher: bool) -> bool {
        match self {
            Req::Yes => true,
            Req::Valid => perms != 0,
            Req::Launcher => launcher,
            Req::Has(i) => perms & (1u64 << i) != 0,
            Req::Not(r) => !r.eval(perms, launcher),
            Req::And(a, b) => a.eval(perms, launcher) && b.eval(perms, launcher),
            Req::Or(a, b) => a.eval(perms, launcher) || b.eval(perms, launcher),
        }
    }
}

fn tokens(s: &str) -> Vec<String> {
    s.replace('(', " ( ").replace(')', " ) ").split_whitespace().map(String::from).collect()
}

fn parse(t: &[String], i: &mut usize) -> Option<Req> {
    let tok = t.get(*i)?.as_str();
    *i += 1;
    match tok {
        ".yes" => Some(Req::Yes),
        ".valid" => Some(Req::Valid),
        ".launcher" => Some(Req::Launcher),
        "(" => {
            let head = t.get(*i)?.clone();
            *i += 1;
            let r = match head.as_str() {
                ".has" => {
                    let n = t.get(*i)?.parse().ok()?;
                    *i += 1;
                    Req::Has(n)
                }
                ".not" => Req::Not(Box::new(parse(t, i)?)),
                ".and" => Req::And(Box::new(parse(t, i)?), Box::new(parse(t, i)?)),
                ".or" => Req::Or(Box::new(parse(t, i)?), Box::new(parse(t, i)?)),
                _ => return None,
            };
            (t.get(*i)?.as_str() == ")").then_some(())?;
            *i += 1;
            Some(r)
        }
        _ => None,
    }
}

/// The body of the list literal `def <name> ... := [ ... ]`, one entry per line.
fn list_lines<'a>(text: &'a str, name: &str) -> Vec<&'a str> {
    let head = alloc::format!("def {name} ");
    let mut out = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        if line.starts_with(&head) {
            inside = true;
            continue;
        }
        if inside {
            if line.starts_with(']') {
                break;
            }
            out.push(line);
        }
    }
    out
}

/// `Formal.Data.capabilityBits`: (name, bit index).
pub(super) fn capability_bits() -> Vec<(String, u32)> {
    list_lines(CAP_BITS, "capabilityBits")
        .into_iter()
        .filter_map(|l| {
            let l = l.trim().trim_end_matches(',');
            let l = l.strip_prefix("(\"")?.strip_suffix(')')?;
            let (name, idx) = l.split_once("\", ")?;
            Some((String::from(name), idx.parse().ok()?))
        })
        .collect()
}

/// `Formal.Data.abiSyscalls`: (number, name).
pub(super) fn abi_syscalls() -> Vec<(u64, String)> {
    list_lines(SYSCALLS, "abiSyscalls")
        .into_iter()
        .filter_map(|l| {
            let l = l.trim().trim_end_matches(',');
            let l = l.strip_prefix('(')?.strip_suffix(')')?;
            let mut parts = l.splitn(3, ", ");
            let nr = parts.next()?.parse().ok()?;
            let _tag = parts.next()?;
            let name = parts.next()?.trim_matches('"');
            Some((nr, String::from(name)))
        })
        .collect()
}

/// `Formal.Data.capTable`: (number, requirement).
pub(super) fn cap_table() -> Vec<(u64, Req)> {
    list_lines(SYSCALLS, "capTable")
        .into_iter()
        .filter_map(|l| {
            let code = l.split(" -- ").next()?.trim().trim_end_matches(',');
            let code = code.strip_prefix('(')?.strip_suffix(')')?;
            let (nr, req) = code.split_once(", ")?;
            let t = tokens(req);
            let mut i = 0;
            let r = parse(&t, &mut i)?;
            (i == t.len()).then_some((nr.parse().ok()?, r))
        })
        .collect()
}

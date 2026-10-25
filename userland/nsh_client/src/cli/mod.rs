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

//! `nsh-verify-record <file> --doc <doc> --fleet <hex> --authority <key>
//! [--ckpt <file>] [--revoked <file>] [--session <hex>] [--subject <hex>]`.
//! An authority key file holds the raw hybrid key, 32 then 1952 bytes; a
//! revoked file holds SHA-512 digests of withdrawn enrolments, 64 bytes
//! each. It prints whose record it is, the verdict and what was not
//! checked offline. Exit codes: 0 complete, 4 complete but holding a
//! provisional `BOX` entry, 3 cut short, 1 refused, 2 usage.

use crate::{verify_record, Attester, Bound, HybridKey, Policy};
use alloc::string::String;
use alloc::vec::Vec;
use args::parse;
use std::fs;

mod args;
mod out;

type Failed = (i32, String);

/// Run the tool over `args` (without the program name), giving the exit
/// code and the text to print.
pub fn run<A: Attester + ?Sized>(args: &[String], attester: &A) -> (i32, String) {
    let Some(a) = parse(args) else { return (2, String::from(out::USAGE)) };
    let read = |p: &String| fs::read(p).map_err(|e| (2, alloc::format!("{p}: {e}")));
    let all = |ps: &[String]| ps.iter().map(read).collect::<Result<Vec<_>, Failed>>();
    let files = (|| Ok::<_, Failed>((read(&a.record)?, read(&a.doc)?, all(&a.authorities)?)))();
    let ((record, doc, keys), ckpts, revoked) = match (files, all(&a.ckpts), all(&a.revoked)) {
        (Ok(f), Ok(c), Ok(r)) => (f, c, r),
        (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => return e,
    };
    let Some(authorities) = keys.iter().map(|k| HybridKey::from_bytes(k)).collect() else {
        return (2, String::from("an authority key is not 1984 bytes"));
    };
    let Some(revoked) = digests(&revoked) else {
        return (2, String::from("a revoked file is not a list of 64-byte digests"));
    };
    let policy = Policy { fleet_id: a.fleet, authorities, revoked };
    let ckpts: Vec<&[u8]> = ckpts.iter().map(Vec::as_slice).collect();
    let expect = Bound { session_id: a.session, transcript: None, subject: a.subject };
    out::show(verify_record(&record, &doc, &ckpts, &expect, &policy, attester))
}

fn digests(files: &[Vec<u8>]) -> Option<Vec<[u8; 64]>> {
    let mut out = Vec::new();
    for f in files {
        if f.is_empty() || f.len() % 64 != 0 {
            return None;
        }
        out.extend(f.chunks_exact(64).filter_map(|d| <[u8; 64]>::try_from(d).ok()));
    }
    Some(out)
}

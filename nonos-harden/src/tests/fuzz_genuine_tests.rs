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

//! The genuine record, receipt and ledger, mutated: no parser panics, and
//! whatever the record parser still accepts names only paths that stay inside
//! the release, digests in the seal's form, and artifact names safe to print.

use super::fuzz::{genuine, mutated, runner};
use crate::hash::is_sha256_hex;
use crate::release::path::{safe, safe_name};
use crate::release::receipt;
use crate::release::record::{self, Item};
use crate::tree::manifest;

#[test]
fn a_mutated_record_is_refused_or_still_names_only_safe_things() {
    let g = genuine();
    runner(1024)
        .run(&mutated(&g.record, 8), |b| {
            let Ok(r) = record::parse(&b) else { return Ok(()) };
            for s in &r.sealed {
                assert!(safe(&s.rel) && safe_name(&s.name), "{} {}", s.name, s.rel);
                match &s.item {
                    Item::File(h) => assert!(is_sha256_hex(h)),
                    Item::Dir(files) => {
                        assert!(files.iter().all(|(p, h)| safe(p) && is_sha256_hex(h)))
                    }
                }
            }
            Ok(())
        })
        .unwrap();
}

#[test]
fn a_mutated_receipt_or_ledger_never_panics_and_the_ledger_stays_safe() {
    let g = genuine();
    runner(1024)
        .run(&mutated(&g.receipt, 8), |b| {
            let _ = receipt::parse(&b);
            Ok(())
        })
        .unwrap();
    runner(1024)
        .run(&mutated(&g.ledger, 8), |b| {
            for (sha, rel) in manifest::parse(&b).unwrap_or_default() {
                assert!(is_sha256_hex(&sha) && safe(&rel), "{sha} {rel}");
            }
            Ok(())
        })
        .unwrap();
}

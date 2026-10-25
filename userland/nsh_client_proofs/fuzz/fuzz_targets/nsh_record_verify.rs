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

//! The record checker. Even first byte: the rest is a record. Odd: it
//! edits a good record (bit 1: footerless; bit 2: with checkpoints) by
//! position and XOR, and a changed record is never complete.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_client::{verify_record, Bound, Verdict};
use nsh_client_proofs::kit::attest::TestAttester;
use nsh_client_proofs::kit::pair::policy;
use nsh_client_proofs::kit::record::{record_doc, Rec};
use std::sync::OnceLock;

struct Good {
    full: Vec<u8>,
    open: Vec<u8>,
    ckpts: Vec<Vec<u8>>,
    doc: Vec<u8>,
}

fn good() -> &'static Good {
    static G: OnceLock<Good> = OnceLock::new();
    G.get_or_init(|| {
        let rec = Rec::good(70);
        let (full, open) = (rec.bytes(), rec.no_footer());
        Good { full, open, ckpts: vec![rec.ckpt(31), rec.ckpt(63)], doc: record_doc(&[0x7A; 64]) }
    })
}

fuzz_target!(|data: &[u8]| {
    let Some((mode, rest)) = data.split_first() else { return };
    let g = good();
    let ckpts: Vec<&[u8]> = match mode & 4 {
        0 => vec![],
        _ => g.ckpts.iter().map(Vec::as_slice).collect(),
    };
    let (attester, policy) = (TestAttester::default(), policy());
    if mode & 1 == 0 {
        let _ = verify_record(rest, &g.doc, &ckpts, &Bound::default(), &policy, &attester);
        return;
    }
    let mut rec = if mode & 2 == 0 { g.full.clone() } else { g.open.clone() };
    for e in rest.chunks_exact(5) {
        let at = u32::from_le_bytes([e[0], e[1], e[2], e[3]]) as usize % rec.len();
        rec[at] ^= e[4];
    }
    let base = if mode & 2 == 0 { &g.full } else { &g.open };
    let v = verify_record(&rec, &g.doc, &ckpts, &Bound::default(), &policy, &attester)
        .map(|r| r.verdict);
    if &rec != base {
        assert!(!matches!(v, Ok(Verdict::Complete { .. })), "a changed record passed");
    } else if mode & 2 == 0 {
        assert!(matches!(v, Ok(Verdict::Complete { entries: 71, .. })), "{v:?}");
    } else if ckpts.is_empty() {
        assert!(v.is_err());
    } else {
        assert_eq!(v, Ok(Verdict::CutShort { seq: 63, boxed: false }));
    }
});

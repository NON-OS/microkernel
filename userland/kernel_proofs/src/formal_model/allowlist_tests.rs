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

//! The model's gate (`Formal.gate`: `allowed c.token.list nr launches` with
//! `Token.list = derive word`) against its shipped allowlist: the
//! bitmaps `allowlist::for_token` derives from a token, read through
//! `allowlist::allowed` and `allowlist::decide`, must give the generated
//! Lean table's answer for every registry slot, both launcher answers, on
//! every capability word class tried; and a number with no slot is refused
//! by both.

extern crate alloc;
use alloc::vec::Vec;

use super::lean_table::{abi_syscalls, cap_table, Req};
use super::words::random_words;
use crate::authorization_tests::token;
use crate::capabilities::Capability;
use crate::syscall::abi::REGISTRY;
use crate::syscall::contract::allowlist::{allowed, decide, for_token};

fn caps_of(w: u64) -> Vec<Capability> {
    Capability::all().iter().copied().filter(|c| w & c.bit() != 0).collect()
}

/// `Formal.derive`: in the ABI, and the table passes.
fn model(
    table: &[(u64, Req)],
    abi: &[(u64, alloc::string::String)],
    nr: u64,
    w: u64,
    l: bool,
) -> bool {
    abi.iter().any(|(n, _)| *n == nr)
        && table.iter().find(|(n, _)| *n == nr).is_some_and(|(_, r)| r.eval(w, l))
}

fn never() -> bool {
    false
}

fn always() -> bool {
    true
}

#[test]
fn the_shipped_allowlist_is_the_model_list() {
    let table = cap_table();
    let abi = abi_syscalls();
    let all = Capability::all().iter().fold(0u64, |w, c| w | c.bit());
    let mut words: Vec<u64> = alloc::vec![0, all];
    for c in Capability::all() {
        words.push(c.bit());
        words.push(all & !c.bit());
    }
    words.extend(random_words(0xa11e, 128).into_iter().map(|w| w & all));
    words.extend(
        random_words(0x5b17, 128)
            .into_iter()
            .map(|w| (1u64 << (w % 40) | 1u64 << ((w >> 8) % 40)) & all),
    );
    let mut checked = 0u32;
    // The kernel checks a token's MAC when it installs the list, and a token
    // that fails it permits nothing; the model's tokens are well formed, so
    // each is signed as install signs it.
    crate::capabilities::token::test_key();
    for &w in &words {
        let mut t = token(caps_of(w));
        crate::capabilities::token::sign_token(&mut t).expect("signing key set");
        let list = for_token(&t);
        for e in REGISTRY.iter().flat_map(|d| d.iter()) {
            for l in [false, true] {
                let m = model(&table, &abi, e.id, w, l);
                assert_eq!(allowed(&list, e.id, l), m, "{} on {w:#x}, launcher {l}", e.name);
                let f: fn() -> bool = if l { always } else { never };
                assert_eq!(decide(&list, e.id, f), m, "{} decide on {w:#x}", e.name);
            }
            checked += 1;
        }
        // A number no slot holds: refused by the list and by the model.
        assert!(!allowed(&list, 0, false) && !allowed(&list, 0, true));
        assert!(!model(&table, &abi, 0, w, true));
    }
    assert!(checked > 50_000, "{checked}");
}

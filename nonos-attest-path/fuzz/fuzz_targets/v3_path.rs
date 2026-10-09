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

//! The path gate against a real enrolled tree admits only the honest trailer
//! of the enrolled slot. Half the inputs flip, extend or cut that trailer, so
//! the search stays near the one byte string that must pass.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_attest_path::{capsule_context, digest_to_bytes, leaf, pad_leaf, verify};
use nonos_attest_path::{Kind, Poseidon, Tree};
use std::sync::OnceLock;

const DEPTH: usize = 8;
struct Policy {
    root: [u8; 32],
    ctx: [u8; 48],
    honest: Vec<u8>,
}

fn policy() -> Option<Policy> {
    let h = Poseidon::new();
    let ctx = capsule_context(&[7u8; 32], 0x183d, 1);
    let mut leaves = vec![leaf(&h, Kind::Capsule, &ctx)?];
    for i in 1..1u32 << DEPTH {
        leaves.push(pad_leaf(&h, &[0x5a; 32], i));
    }
    let tree = Tree::commit(&h, &leaves, DEPTH)?;
    let root = digest_to_bytes(&tree.root()?);
    Some(Policy { root, ctx, honest: tree.trailer(0)? })
}

static POLICY: OnceLock<Option<Policy>> = OnceLock::new();

fuzz_target!(|data: &[u8]| {
    let Some(p) = POLICY.get_or_init(policy) else {
        return;
    };
    assert!(verify(&p.root, DEPTH, Kind::Capsule, &p.ctx, &p.honest), "honest refused");
    let Some((&mode, rest)) = data.split_first() else {
        return;
    };
    let trailer: Vec<u8> = if mode & 1 == 0 {
        rest.to_vec()
    } else {
        let mut t = p.honest.clone();
        let (flip, tail) = rest.split_at(rest.len().min(t.len()));
        for (x, b) in t.iter_mut().zip(flip) {
            *x ^= b;
        }
        if mode & 2 != 0 {
            t.extend_from_slice(tail);
        } else {
            t.truncate(t.len() - usize::from(mode >> 2).min(t.len()));
        }
        t
    };
    if verify(&p.root, DEPTH, Kind::Capsule, &p.ctx, &trailer) {
        assert!(trailer == p.honest, "a trailer other than the honest one was admitted");
    }
});

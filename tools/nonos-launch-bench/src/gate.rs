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

//! The per-syscall gate, the parts of it that run without a machine: the
//! registry lookup, the token MAC, the capability decision, the witness the
//! resolver builds, and the audit name lookup. `before` is the code before
//! this change (the two linear scans are copied from it verbatim; the tables
//! are still in the tree); `after` is this branch. The locks around them
//! (PCB token RwLock, revocation set, ASID lookup) are the same before and
//! after and are not timed.

use std::hint::black_box;
use std::sync::Arc;
use std::time::Instant;

use crate::capabilities::bits::caps_to_bits;
use crate::capabilities::token::material::{mac64, token_material};
use crate::capabilities::{Capability, CapabilityToken};
use crate::syscall::abi::{self, REGISTRY};
use crate::syscall::contract::allowlist::{allowed, decide, for_token};
use crate::syscall::contract::tables_allow;
use crate::syscall::numbers::SyscallNumber;

const KEY: [u8; 32] = [0x5a; 32];
const CALLS: u32 = 2_000_000;

fn old_lookup_id(id: u64) -> Option<SyscallNumber> {
    for slice in REGISTRY {
        for entry in *slice {
            if entry.id == id {
                return Some(entry.variant);
            }
        }
    }
    None
}

fn old_lookup_name(id: u64) -> Option<&'static str> {
    for slice in REGISTRY {
        for entry in *slice {
            if entry.id == id {
                return Some(entry.name);
            }
        }
    }
    None
}

fn ct_eq_64(a: &[u8; 64], b: &[u8; 64]) -> bool {
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

pub(crate) fn verify_token(t: &CapabilityToken) -> bool {
    let mat = token_material(t, caps_to_bits(&t.permissions));
    ct_eq_64(&mac64(&KEY, &mat), &t.signature)
}

fn app_token() -> CapabilityToken {
    let mut t = CapabilityToken::with_caps(&[
        Capability::CoreExec,
        Capability::IPC,
        Capability::Memory,
        Capability::FileSystem,
        Capability::GraphicsDisplayQuery,
        Capability::GraphicsSurfaceCreate,
        Capability::GraphicsSurfaceMap,
        Capability::GraphicsPresent,
    ]);
    t.owner_module = 42;
    t.nonce = 0x1234;
    let mat = token_material(&t, caps_to_bits(&t.permissions));
    t.signature = mac64(&KEY, &mat);
    t
}

const HOT: [SyscallNumber; 7] = [
    SyscallNumber::MkSurfacePresent,
    SyscallNumber::MkInputEventDrain,
    SyscallNumber::MkIpcSend,
    SyscallNumber::MkIpcRecv,
    SyscallNumber::MkFutexWait,
    SyscallNumber::MkFutexWake,
    SyscallNumber::MkTimeMonotonic,
];

fn per_call<F: FnMut(u64) -> bool>(mut f: F) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..5 {
        let start = Instant::now();
        let mut ok = 0u32;
        for i in 0..CALLS {
            let nr = HOT[(i as usize) % HOT.len()] as u64;
            ok += u32::from(f(black_box(nr)));
        }
        black_box(ok);
        best = best.min(start.elapsed().as_nanos() as f64 / f64::from(CALLS));
    }
    best
}

/// Runs the gate section; returns the default build's modelled gate cost.
pub fn run() -> f64 {
    let token = app_token();
    let arc = Arc::new(token.clone());
    let list = for_token(&token);
    for nr in HOT {
        let n = nr as u64;
        if allowed(&list, n, false) != tables_allow(&token, nr) {
            println!("gate: list and tables disagree on {nr:?}; not timed");
            return 0.0;
        }
    }
    println!("# syscall gate, per call over a mix of {} hot syscalls (best of 5 x {CALLS})", HOT.len());
    let parts: [(&str, f64, f64); 5] = [
        (
            "registry lookup (linear scan -> hash)",
            per_call(|n| old_lookup_id(n).is_some()),
            per_call(|n| abi::lookup_id(n).is_some()),
        ),
        (
            "capability decision (tables -> allowlist)",
            per_call(|n| old_lookup_id(n).is_some_and(|s| tables_allow(&token, s))),
            per_call(|n| decide(&list, n, || false)),
        ),
        (
            "witness (Arc + token clone -> Arc)",
            per_call(|n| {
                let a = Arc::clone(&arc);
                black_box(a.as_ref().clone()).owner_module == n
            }),
            per_call(|n| black_box(Arc::clone(&arc)).owner_module == n),
        ),
        (
            "audit name (linear scan -> hash)",
            per_call(|n| old_lookup_name(n).is_some()),
            per_call(|n| abi::lookup_name(n).is_some()),
        ),
        ("token MAC, 2 x keyed BLAKE3 (unchanged)", per_call(|_| verify_token(&token)), per_call(|_| verify_token(&token))),
    ];
    let every: Vec<u64> = REGISTRY.iter().flat_map(|s| s.iter().map(|e| e.id)).collect();
    let spread = |f: &dyn Fn(u64) -> bool| {
        let start = Instant::now();
        let mut ok = 0u32;
        for i in 0..CALLS {
            ok += u32::from(f(black_box(every[(i as usize) % every.len()])));
        }
        black_box(ok);
        start.elapsed().as_nanos() as f64 / f64::from(CALLS)
    };
    println!(
        "{:<44} before={:>8.1} ns  after={:>8.1} ns  host",
        "registry lookup, every syscall in turn",
        spread(&|n| old_lookup_id(n).is_some()),
        spread(&|n| abi::lookup_id(n).is_some())
    );
    let (mut before, mut after) = (0.0, 0.0);
    for (what, b, a) in parts {
        println!("{what:<44} before={b:>8.1} ns  after={a:>8.1} ns  host");
        before += b;
        after += a;
    }
    let mac = parts[4].2;
    println!("{:<44} before={before:>8.1} ns  after={:>8.1} ns  modelled", "gate total, default build", after - mac);
    println!("{:<44} before={before:>8.1} ns  after={after:>8.1} ns  modelled", "gate total, nonos-syscall-token-mac-every-call");
    after - mac
}

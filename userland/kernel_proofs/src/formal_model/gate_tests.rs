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


//! Syscall-gate completeness (`Formal.syscall_gate_completeness`). The model's
//! gate is `Formal.Data.capTable`, generated from `cap_table`; here it is
//! evaluated on the same tokens the real `cap_table::is_allowed` sees, for
//! every registered syscall, and the two must agree everywhere. The
//! generated capability bits and ABI list are held to the real enum and
//! registry the same way.

extern crate alloc;
use alloc::format;
use alloc::vec::Vec;

use super::lean_table::{abi_syscalls, cap_table, capability_bits};
use super::words::random_words;
use crate::authorization_tests::token;
use crate::capabilities::Capability;
use crate::syscall::abi::REGISTRY;
use crate::syscall::contract::{is_allowed, is_allowed_as_shell};

fn word(caps: &[Capability]) -> u64 {
    caps.iter().fold(0, |w, c| w | c.bit())
}

fn caps_of(w: u64) -> Vec<Capability> {
    Capability::all().iter().copied().filter(|c| w & c.bit() != 0).collect()
}

#[test]
fn generated_capability_bits_match_the_enum() {
    let lean = capability_bits();
    let real = Capability::all();
    assert_eq!(lean.len(), real.len());
    for (c, (name, idx)) in real.iter().zip(lean.iter()) {
        assert_eq!(&format!("{c:?}"), name);
        assert_eq!(c.bit(), 1u64 << idx, "{name}");
    }
}

#[test]
fn generated_abi_list_matches_the_registry() {
    let lean = abi_syscalls();
    let mut real: Vec<(u64, &str)> =
        REGISTRY.iter().flat_map(|d| d.iter().map(|e| (e.id, e.name))).collect();
    real.sort();
    let mut lean_sorted: Vec<(u64, &str)> = lean.iter().map(|(n, s)| (*n, s.as_str())).collect();
    lean_sorted.sort();
    assert_eq!(lean_sorted, real);
}

#[test]
fn the_model_gate_is_the_kernel_gate() {
    let table = cap_table();
    let all = word(Capability::all());
    let mut words: Vec<u64> = Vec::new();
    words.push(0);
    words.push(all);
    for c in Capability::all() {
        words.push(c.bit());
        words.push(all & !c.bit());
    }
    words.extend(random_words(0x9a7e, 256).into_iter().map(|w| w & all));
    // Sparse words: two or three bits, the shape most manifests have.
    words.extend(random_words(0x5a75, 256).into_iter().map(|w| {
        let a = 1u64 << (w % 40);
        let b = 1u64 << ((w >> 8) % 40);
        let c = if w & 1 == 0 { 1u64 << ((w >> 16) % 40) } else { 0 };
        (a | b | c) & all
    }));
    let mut checked = 0u32;
    for entry in REGISTRY.iter().flat_map(|d| d.iter()) {
        let req = table.iter().find(|(nr, _)| *nr == entry.id).map(|(_, r)| r);
        for &w in &words {
            let tok = token(caps_of(w));
            let model = req.is_some_and(|r| r.eval(w, false));
            let model_shell = req.is_some_and(|r| r.eval(w, true));
            assert_eq!(is_allowed(&tok, entry.variant), model, "{} on {w:#x}", entry.name);
            assert_eq!(is_allowed_as_shell(&tok, entry.variant), model_shell, "{} as shell", entry.name);
            checked += 1;
        }
    }
    assert!(checked > 100_000, "{checked}");
}

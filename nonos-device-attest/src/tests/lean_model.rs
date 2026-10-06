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

//! The STARK lane's Lean model of this circuit, `lean/Shield/Device.lean`, read
//! from the text the pinned commit ships, so the tie below always compares the
//! circuit with the model of the same commit. The circuit's side is read here
//! too, in the model's terms.

use alloc::vec::Vec;
use stark_proofs::crypto::stark::air::{Air, ShieldRegion, WiredMultiGen};
use stark_proofs::lean_text::{nat, table, DEVICE};

/// A `def name : Nat := value` of the model.
pub(super) fn n(name: &str) -> usize {
    nat(DEVICE, name)
        .and_then(|v| usize::try_from(v).ok())
        .unwrap_or_else(|| panic!("the model has no Nat {name}"))
}

/// A literal list or table of the model, its tuples flattened in order.
pub(super) fn list(name: &str) -> Vec<usize> {
    let t = table(DEVICE, name).unwrap_or_else(|| panic!("the model has no table {name}"));
    t.into_iter().map(|v| v as usize).collect()
}

/// The numbers of a definition written as an expression, from `:=` to the
/// blank line that ends it, such as `wire`'s two region starts.
pub(super) fn numbers_in(name: &str) -> Vec<usize> {
    let key = alloc::format!("def {name} ");
    let at = DEVICE.find(&key).unwrap_or_else(|| panic!("the model has no def {name}"));
    let rest = &DEVICE[at..];
    let open = rest.find(":=").unwrap_or_else(|| panic!("def {name} has no body"));
    let end = rest[open..].find("\n\n").map_or(rest.len(), |e| open + e);
    rest[open..end].split(|c: char| !c.is_ascii_digit()).filter_map(|t| t.parse().ok()).collect()
}

/// Each region's rows, and each chain's schedule and pinned cells.
pub(super) fn regions(air: &WiredMultiGen) -> Vec<(usize, Option<(usize, usize, usize)>, usize)> {
    let shape = |g: &ShieldRegion| match g {
        ShieldRegion::Membership(m) => (Air::rows(m), Some(m.schedule()), Air::boundary(m).len()),
        ShieldRegion::Publics(p) => (Air::rows(p), None, p.words.len()),
        _ => (0, None, usize::MAX),
    };
    air.regions().iter().map(shape).collect()
}

/// The slots a permutation moves: the cells it binds, all others fixed.
pub(super) fn moved(sigma: &[usize]) -> Vec<(usize, usize)> {
    sigma.iter().enumerate().filter(|&(i, &s)| i != s).map(|(i, &s)| (i, s)).collect()
}

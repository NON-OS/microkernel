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

//! The periodic cache the build ships, checked whole and against the tree built here.

use std::io::Write;

use crate::serial::{line, TAG};

pub(crate) static SHIPPED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/periodic.top"));

/// The periodic root check on the shipped file: taken whole, refused with
/// one bit of its root changed.
pub(crate) fn shipped_cache(out: &mut impl Write) -> bool {
    if SHIPPED.is_empty() {
        println!("{TAG} cache: none shipped in this build; the first proof builds it");
        return true;
    }
    let taken = nox_prover::load_cache(SHIPPED).is_ok();
    let mut touched = SHIPPED.to_vec();
    if let Some(last) = touched.last_mut() {
        *last ^= 1;
    }
    let refused = nox_prover::load_cache(&touched).is_err();
    line(out, taken, &format!("cache: the shipped {} bytes carry PERIODIC_ROOT", SHIPPED.len()))
        & line(out, refused, "cache: the same file with one bit of its root changed is refused")
}

/// The tree a proof from nothing built, held to the shipped cache when this
/// build ships one.
pub(crate) fn built_is_shipped(out: &mut impl Write, name: &str, top: &[u8]) {
    if !SHIPPED.is_empty() {
        line(
            out,
            top == SHIPPED,
            &format!("{}: the tree built here is the shipped cache, byte for byte", name),
        );
    }
}

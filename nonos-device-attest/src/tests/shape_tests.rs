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

//! The circuit's shape, pinned: the numbers the STARK lane's Lean model of the
//! device circuit is written from. A change to any of them is a change to that
//! model and must move both together.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::crypto::stark::field::Fp;

use crate::circuit::build;
use crate::params::{LOG_TRACE, SHIPPED_DEVICE_DEPTH};
use crate::registry::MAX_DEVICE_DEPTH;
use crate::statement::Statement;

pub(super) fn statement(depth: usize) -> Statement {
    Statement {
        boot_root: [Fp::ONE; 4],
        kernel_root: [Fp::ONE; 4],
        device_root: [Fp::ONE; 4],
        device_depth: depth,
        scope: [Fp::ONE; 2],
        context: [Fp::ONE; 4],
        tag: [Fp::ONE; 4],
    }
}

/// Every registry depth the circuit accepts builds at the certified length, so
/// no device proof ever comes out at a shape the certificate does not cover.
#[test]
fn every_registry_depth_builds_at_the_certified_length() {
    for depth in 1..=MAX_DEVICE_DEPTH {
        let b = build(&statement(depth), None)
            .unwrap_or_else(|| panic!("depth {depth} does not build"));
        assert_eq!(b.wired.log_trace_len(), LOG_TRACE, "depth {depth}");
    }
}

/// The builder refuses any length but the certified one. A registry this deep
/// stacks past `2^14`, and nothing is built for it: no proof exists at a shape
/// the certificate was not built for.
#[test]
fn a_circuit_at_any_other_length_is_refused() {
    let b = build(&statement(300), None);
    assert!(b.is_none(), "a depth-300 registry built a circuit");
}

/// The masking and product columns the shipped shape carries beside the regions
/// the Lean model describes; `lean_tie_tests` holds the rest to the model.
#[test]
fn the_shipped_shape_masks_two_columns_and_runs_two_products() {
    let b = build(&statement(SHIPPED_DEVICE_DEPTH), None).expect("builds");
    let w = b.wired.wired();
    assert_eq!((w.mask_columns(), w.product_columns()), (2, 2));
}

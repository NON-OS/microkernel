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

//! The tie to the Lean model: the shipped circuit, built at the shipped depth,
//! has the sizes, kinds, rows, offsets and wire `Device.lean` proves things of.

use alloc::vec::Vec;
use stark_proofs::crypto::stark::air::Air;
use stark_proofs::shield::wire::offsets;
use stark_proofs::shield::wire_class::tie;
use stark_proofs::shield::wire_pack::{packed_groups, CAP};

use super::lean_model::{list, moved, n, numbers_in, regions};
use super::shape_tests::statement;
use crate::circuit::build;
use crate::params::{BOOT_DEPTH, KERNEL_DEPTH, SHIPPED_DEVICE_DEPTH};

#[test]
fn the_shipped_shape_is_the_one_the_lean_model_describes() {
    let depths = (n("registryDepth"), n("bootDepth"), n("kernelDepth"));
    assert_eq!(depths, (SHIPPED_DEVICE_DEPTH, BOOT_DEPTH, KERNEL_DEPTH));
    let b = build(&statement(SHIPPED_DEVICE_DEPTH), None).expect("the shipped depth builds");
    let air = &b.wired;
    assert_eq!((air.log_trace_len() as usize, air.trace_width()), (n("logTrace"), n("width")));
    let kinds = air.wired().kind_map().into_iter().flat_map(|(a, b, c, d, e)| [a, b, c, d, e]);
    assert_eq!(kinds.collect::<Vec<_>>(), list("kindMap"));

    let rounds = n("rounds");
    let compressions = [1 + depths.1, 1 + depths.2, 1 + depths.0, 1];
    let pins = list("pinnedLanes").len() + list("wiredLanes").len();
    let mut want: Vec<_> =
        compressions.iter().map(|&c| ((c + 1) * rounds, Some((rounds, c, 1)), pins)).collect();
    want.push((n("padRows"), None, 0));
    let shapes = regions(air);
    assert_eq!(shapes, want);
    let rows: Vec<usize> = shapes.iter().map(|s| s.0).collect();
    assert_eq!(rows[..4], [320, 320, 704, 64]);
    let (off, span) = offsets(&rows);
    assert_eq!(off, list("offsets"));

    /* The wire, (640, l) to (1344, l) for lanes 1 to 4, and nothing more. */
    let ends = numbers_in("wire");
    assert_eq!(ends, [off[2], off[3]]);
    let classes: Vec<_> =
        list("wiredLanes").iter().map(|&l| tie(&[(ends[0], l), (ends[1], l)])).collect();
    let model: Vec<_> = packed_groups(span, &classes, CAP)
        .iter()
        .map(|g| (g.wired_cols.clone(), moved(&g.sigma)))
        .collect();
    let params = air.wired().group_params().into_iter().map(|g| g.0);
    let sigmas = air.wired().group_sigmas().into_iter().map(moved);
    assert_eq!(params.zip(sigmas).collect::<Vec<_>>(), model);
}

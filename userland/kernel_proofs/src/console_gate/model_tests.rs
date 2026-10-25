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

//! The gate with TAKERS loaded before GONE, searched over every
//! interleaving of two drawers trying two draws each, one or two takeovers
//! made ready or refused, and bootfb's hold set and cleared, the clear also
//! taken while the hold's wait still runs. Each search must cover at least
//! the states and runs it covered when written.

use super::actors::World;
use super::drawer_steps::probe;
use super::gate::Gate;
use super::search::search;
use super::world::Flags;

#[test]
fn the_kernels_gate_keeps_every_rule_on_every_interleaving() {
    let g = Gate::KERNEL;
    for (takers, states, steps, runs) in
        [(1, 10230, 31739, 11066748025173252), (2, 51132, 194524, 4287978063619306896)]
    {
        let c = search(&g, World::new(&g, 2, takers, true), &[]);
        let c = c.unwrap_or_else(|f| panic!("{f:?}"));
        assert!(c.states >= states && c.steps >= steps && c.runs >= runs, "{takers}: {c:?}");
    }
}

#[test]
fn stopped_alone_is_held_or_gone_or_a_takeover_under_way() {
    for g in [Gate::KERNEL, Gate::GONE_FIRST] {
        for bits in 0u8..12 {
            let (held, gone, takers) = (bits & 1 != 0, bits & 2 != 0, bits >> 2);
            let f = Flags { held, gone, takers, draws: 0 };
            let stop = held || gone || takers > 0;
            assert_eq!(g.reads.iter().any(|&r| f.stop(r)), stop, "{f:?}");
            assert_eq!(probe(&g, &f), !stop, "{f:?}");
        }
    }
}

#[test]
fn the_check_before_the_count_only_spares_a_stopped_console_its_count() {
    let g = Gate { first_check: false, ..Gate::KERNEL };
    let c = search(&g, World::new(&g, 2, 2, true), &[]).unwrap_or_else(|f| panic!("{f:?}"));
    assert!(c.states >= 21972 && c.steps >= 76276 && c.runs >= 771917557087588, "{c:?}");
}

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
//! Vol 12's DisplayPort PLL table (p. 178, 179): each row's P, Q and K,
//! and the rates and references it does not list.

use nonos_intel_xelp_display::pll::dp::dividers;

const RATES: [u32; 8] = [1620, 2160, 2700, 3240, 4320, 5400, 6480, 8100];

#[test]
fn every_rows_p_q_and_k() {
    // Vol 12 p. 178, 179: Pdiv, Kdiv and Qdiv for 1.62 to 8.1 Gbit/s.
    let pkq =
        [(5, 2, 1), (2, 2, 2), (3, 2, 1), (5, 1, 1), (2, 2, 1), (3, 1, 1), (3, 1, 1), (2, 1, 1)];
    for (r, (p, k, q)) in RATES.iter().zip(pkq) {
        for f in [24_000, 19_200] {
            let d = dividers(*r, f).unwrap();
            assert_eq!((d.p, d.k, d.q), (p, k, q), "{r} at {f}");
        }
    }
}

#[test]
fn unlisted_rates_and_references_refused() {
    for r in [0, 1600, 2600, 2701, 10000] {
        assert!(dividers(r, 24_000).is_none(), "{r}");
    }
    for f in [0, 25_000, 19_000, 100_000] {
        assert!(dividers(2700, f).is_none(), "{f}");
    }
}

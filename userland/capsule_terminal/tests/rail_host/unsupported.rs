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

//! Figures NONOS has no source for stay marked unsupported.

use crate::rail::metrics::Sample;
use crate::rail::net_decode::decode_lease;
use crate::rail::value::Metric;
use crate::support::lease;

#[test]
fn the_figures_nonos_cannot_measure_stay_unsupported() {
    let s = Sample::EMPTY;
    let up = decode_lease(&lease(3, [1, 2, 3, 4], 8));
    assert!(
        up.ipv6 == Metric::Unsupported
            && up.rx_bps == Metric::Unsupported
            && up.tx_bps == Metric::Unsupported
    );
    assert!(s.disk.total_kb == Metric::Unsupported && s.disk.used_kb == Metric::Unknown);
    assert!(s.load_avg == Metric::Unknown, "load has a source, it is merely unread");
    assert_eq!(Metric::Known(7u32).value(), Some(7));
}

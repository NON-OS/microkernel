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

//! Unmeasured figures render as a dash.

use crate::paint::rail_fmt::{load3_into, mib_into};
use crate::paint::rail_metric::{one, pair, DASH};
use crate::rail::mem::summarize;
use crate::rail::metrics::Proc;
use crate::rail::value::Metric;
use crate::term::util::format_u64;

/// A plain count, the simplest formatter `one` can be handed.
fn count_into(buf: &mut [u8], v: u32) -> usize {
    format_u64(v as u64, buf)
}

#[test]
fn a_metric_with_no_source_renders_as_a_dash_rather_than_a_zero() {
    let mut b = [0u8; 48];
    assert_eq!(one(&mut b, Metric::<u64>::Unknown, mib_into), DASH);
    assert_eq!(one(&mut b, Metric::<u64>::Unsupported, mib_into), DASH);
    assert_eq!(one(&mut b, Metric::Known(0u32), count_into), "0", "a measured zero is not a dash");
    assert_eq!(one(&mut b, Metric::<u32>::Unsupported, count_into), DASH);
}

#[test]
fn a_pair_dashes_each_half_independently() {
    let mut b = [0u8; 48];
    let m = summarize(&[Proc { pid: 1, mem_kb: 3952, ..Proc::EMPTY }], 0);
    assert_eq!(pair(&mut b, m.used_kb, m.total_kb, mib_into), "3.8 MB / —");
    assert_eq!(pair(&mut b, m.total_kb, m.swap_used_kb, mib_into), "— / —");
    assert_eq!(
        pair(&mut b, Metric::Known(0u64), Metric::Known(1024u64), mib_into),
        "0.0 MB / 1.0 MB"
    );
}

#[test]
fn an_unread_load_is_a_dash_and_a_read_one_never_is() {
    let mut b = [0u8; 48];
    assert_eq!(one(&mut b, Metric::<[u64; 3]>::Unknown, load3_into), DASH);
    assert_eq!(one(&mut b, Metric::Known([0u64; 3]), load3_into), "0.00 0.00 0.00");
}

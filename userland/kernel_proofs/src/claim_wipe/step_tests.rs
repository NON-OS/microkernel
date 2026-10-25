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

//! Whether a wipe writes its reset, over every look and the clock around the
//! pending-transactions bound: never without a method, a marker and D0 (as
//! built, a function outside D0 is left unreset, not woken); held off while
//! work is pending inside the bound; else written into the method's window.

use super::due_rule::{due, Due};
use super::marker_rule::Marker;
use super::method_rule::Method;
use super::phase_rule::{Phase, FLR_WAIT, PM_WAIT, TP_BOUND};
use super::start_rule::{start, written, Do, Look};
use super::step_cases::{looks, METHODS};

const SINCE: [u64; 3] = [0, 5_000_123, u64::MAX - 2 * TP_BOUND];

#[test]
fn a_reset_is_written_only_in_d0_with_a_method_and_a_marker_past_pending_work() {
    let mut seen = [0u32; 5];
    for (m, look) in looks() {
        for since in SINCE {
            let around = [TP_BOUND - 1, TP_BOUND, TP_BOUND + 1, 0].map(|d| since + d);
            for now in since.checked_sub(1).into_iter().chain(around) {
                let held_off = look.tp && (now < since || now - since < TP_BOUND);
                let want = match (m, look.marker) {
                    (Some(m), Some(k)) if look.in_d0 && !held_off => Do::Write(m, k),
                    (Some(_), Some(_)) if look.in_d0 => Do::Stay,
                    _ => Do::Unreset,
                };
                assert_eq!(start(m, look, since, now), want, "{m:?} {look:?} {since} {now}");
                let kind = [Do::Unreset, Do::Stay].iter().position(|&d| d == want).unwrap_or(2);
                seen[kind] += 1;
                seen[3] += (want == Do::Stay && now < since) as u32;
                seen[4] += (kind == 2 && look.tp && now == since + TP_BOUND) as u32;
            }
        }
    }
    assert_eq!(seen, [504, 48, 120, 12, 18], "unreset, held off, written; behind, at the bound");
}

#[test]
fn pending_transactions_hold_the_reset_off_until_the_bound_and_no_longer() {
    let look = Look { in_d0: true, tp: true, marker: Some(Marker::IntxDisable) };
    let at = |now| start(Some(Method::AfFlr), look, 1_000, now);
    assert_eq!(at(1_000 + TP_BOUND - 1), Do::Stay);
    assert_eq!(at(1_000 + TP_BOUND), Do::Write(Method::AfFlr, Marker::IntxDisable));
    assert_eq!(at(999), Do::Stay, "a clock behind the owing reads as not yet");
}

#[test]
fn a_written_reset_is_not_looked_at_inside_its_quiet_window() {
    for (now, m) in [0, 77, u64::MAX - FLR_WAIT].into_iter().flat_map(|t| METHODS.map(|m| (t, m))) {
        let (phase, wait, next) = match m {
            Method::Flr | Method::AfFlr => (Phase::Flr { at: now }, FLR_WAIT, Due::Check),
            Method::PmCycle => (Phase::D3 { at: now }, PM_WAIT, Due::ToD0),
        };
        assert_eq!(written(m, now), phase, "{m:?} at {now}");
        assert_eq!((due(phase, now + wait - 1), due(phase, now + wait)), (Due::Wait, next));
    }
}

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
//! Discovery against a peripheral one of whose answers lies: an octet
//! changed, cut short, run on, or a handle swapped. Whatever it says, the
//! host ends ready or failed, never waiting for good, and every report it
//! keeps lies inside one of the device's HID services, and its CCCD
//! inside its own characteristic.

use super::gatt_lies::{device, lie, services};
use super::rng::each;
use crate::sim::Run;
use nonos_bt_hogp::host::Outcome;

#[test]
fn no_lying_answer_points_the_host_outside_its_services() {
    let honest: Vec<usize> =
        (0..4).map(|k| Run::new(device(k)).go_lying(usize::MAX, |_| {}).1).collect();
    each(|rng| {
        let k = rng.below(4);
        let ranges = services(&device(k));
        let mut run = Run::new(device(k));
        let n = rng.below(honest[k]);
        let (out, _) = run.go_lying(n, |a| lie(rng, a));
        assert!(
            matches!(out, Some(Outcome::Ready { .. } | Outcome::Failed(_))),
            "answer {n}: {out:?}"
        );
        for s in run.host.sources() {
            let inside = |h: u16| ranges.iter().any(|r| (r.0..=r.1).contains(&h));
            assert!(inside(s.value) && inside(s.end) && s.cccd.is_none_or(inside), "{s:?}");
            // Its CCCD is its own: after its value, by its last handle.
            assert!(s.cccd.is_none_or(|c| s.value < c && c <= s.end), "{s:?}");
        }
    });
}

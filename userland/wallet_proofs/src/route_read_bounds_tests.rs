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

//! The routed reader's bounds: no answer, an oversized flight, a drip, a break.

use super::route_script::{Script, BOUNDS};
use crate::wallet::net::routed_read::{read_routed, Bounds, Source};

#[test]
fn nothing_at_all_is_an_error_after_the_first_wait() {
    let mut s = Script::new(vec![]);
    assert_eq!(read_routed(&mut s, BOUNDS, |_| false), None);
    assert_eq!(s.now, 60_000);
    let mut closed = Script::new(vec![None]);
    assert_eq!(read_routed(&mut closed, BOUNDS, |_| false), None);
}

#[test]
fn more_than_a_flight_holds_is_refused() {
    let mut s = Script::new(vec![Some((10, vec![1u8; 60])), Some((10, vec![2u8; 10]))]);
    assert_eq!(read_routed(&mut s, BOUNDS, |_| false), None);
}

#[test]
fn a_drip_cannot_hold_the_wallet_past_the_total() {
    let steps = (0..1_000).map(|_| Some((7_999, vec![1u8]))).collect();
    let mut s = Script::new(steps);
    let bounds = Bounds { max: 4_096, ..BOUNDS };
    let got = read_routed(&mut s, bounds, |_| false);
    assert!(got.is_some());
    assert!(s.now <= 90_000 + 8_000, "stopped by the total: {}", s.now);
}

/// A stream that breaks once its script has run out.
struct Breaks(Script);

impl Source for Breaks {
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        self.0.broke = self.0.steps.is_empty();
        self.0.read_wait(into, wait_ms)
    }

    fn now_ms(&self) -> i64 {
        self.0.now
    }
}

#[test]
fn a_stream_that_breaks_keeps_what_came_before_and_nothing_else() {
    let mut s = Breaks(Script::new(vec![Some((10, b"part".to_vec()))]));
    assert_eq!(read_routed(&mut s, BOUNDS, |_| false), Some(b"part".to_vec()));
    let mut at_once = Breaks(Script::new(vec![]));
    assert_eq!(read_routed(&mut at_once, BOUNDS, |_| false), None);
}

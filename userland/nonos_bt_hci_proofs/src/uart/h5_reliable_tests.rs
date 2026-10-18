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

//! The reliable layer (Vol 4, Part D, "Reliable Packets") over a line
//! that loses packets: every command arrives once, in order.

use super::h5_pair::Pair;

#[test]
fn every_command_arrives_once_and_in_order_over_a_lossy_line() {
    for lose in [0, 10, 30, 50] {
        let mut p = Pair::new(lose);
        assert!(p.until(5_000, |p| p.a.active() && p.b.active()), "established, lose {lose}");
        let mut got = Vec::new();
        let mut next = 0u8;
        for _ in 0..50_000 {
            if next < 40 && p.a.send(1, &[next]) {
                next += 1;
            }
            p.tick();
            got.extend(p.b.inbox.drain(..).map(|(k, v)| (k, v[0])));
            if got.len() == 40 && p.a.unacked() == 0 {
                break;
            }
        }
        let want: Vec<(u8, u8)> = (0..40).map(|i| (1, i)).collect();
        assert_eq!(got, want, "lose {lose} in 100");
        assert!(!p.a.lost(p.now));
    }
}

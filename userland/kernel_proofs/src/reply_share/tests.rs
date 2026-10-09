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

use super::share::{admits, MAX_PER_CALLER, MAX_PER_SERVICE};

/// Callers in turn ask for a place; returns how many each got.
fn fill(callers: &[u32], rounds: usize) -> Vec<(u32, usize)> {
    let mut queue: Vec<u32> = Vec::new();
    for _ in 0..rounds {
        for &c in callers {
            let mine = queue.iter().filter(|&&p| p == c).count();
            if admits(queue.len(), mine) {
                queue.push(c);
            }
        }
    }
    callers.iter().map(|&c| (c, queue.iter().filter(|&&p| p == c).count())).collect()
}

#[test]
fn one_caller_holds_at_most_its_share() {
    assert_eq!(fill(&[7], 100), vec![(7, MAX_PER_CALLER)]);
}

#[test]
fn a_flooding_caller_leaves_room_for_the_rest() {
    let got = fill(&[7, 8], 100);
    assert_eq!(got, vec![(7, MAX_PER_CALLER), (8, MAX_PER_CALLER)]);
    // After one caller took its whole share, the service still has room.
    assert!(admits(MAX_PER_CALLER, 0));
}

#[test]
fn the_service_holds_at_most_its_queue() {
    let callers: Vec<u32> = (1..=20).collect();
    let total: usize = fill(&callers, 100).iter().map(|c| c.1).sum();
    assert_eq!(total, MAX_PER_SERVICE);
    assert!(!admits(MAX_PER_SERVICE, 0));
}

#[test]
fn the_share_leaves_several_callers_room() {
    const { assert!(MAX_PER_CALLER >= 2 && MAX_PER_CALLER * 4 <= MAX_PER_SERVICE) };
    assert!(admits(0, 0));
    assert!(admits(MAX_PER_SERVICE - 1, MAX_PER_CALLER - 1));
    assert!(!admits(MAX_PER_SERVICE - 1, MAX_PER_CALLER));
}

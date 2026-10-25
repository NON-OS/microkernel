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

//! terminate()'s once (src/security/zerostate/once.rs): of any number of
//! callers, racing or in turn, exactly one goes on to the leave phase, the
//! stop and the wipe; a second would stop the CPUs under the first's wipe.

#[path = "../../../../src/security/zerostate/once.rs"]
mod once;

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicBool;
    use std::sync::{Arc, Barrier};

    use super::once::first;

    #[test]
    fn the_first_caller_goes_on_and_every_later_one_parks() {
        let flag = AtomicBool::new(false);
        assert!(first(&flag), "the first caller was parked");
        assert!(!first(&flag), "a second caller went on to a second wipe");
        assert!(!first(&flag));
    }

    #[test]
    fn of_callers_racing_exactly_one_goes_on() {
        for _ in 0..200 {
            let flag = Arc::new(AtomicBool::new(false));
            let start = Arc::new(Barrier::new(8));
            let callers: Vec<_> = (0..8)
                .map(|_| {
                    let (flag, start) = (Arc::clone(&flag), Arc::clone(&start));
                    std::thread::spawn(move || {
                        start.wait();
                        first(&flag)
                    })
                })
                .collect();
            let through = callers.into_iter().map(|c| c.join().unwrap()).filter(|&on| on).count();
            assert_eq!(through, 1, "{through} callers went on to the wipe");
        }
    }
}

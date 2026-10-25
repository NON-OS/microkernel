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

//! Who a function's last holder was: the least `driver.` name a claimant
//! serves that fits, over every list of up to five names from the pool,
//! repeats included, so every order of every such set gives one answer;
//! every foreign or over-long name ignored even where it sorts first. As
//! built, the bare root `driver.` counts as a name that fits.

use super::holder_pool::{fits, pool};
use super::holder_rule::{name_of, NAME_MAX};

#[test]
fn the_least_fitting_driver_name_whatever_the_order() {
    let names = pool();
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    // Answers: none, the bare root, one exactly NAME_MAX long; and a foreign
    // name, an over-long one, sorting below the answer and ignored.
    let mut seen = [0u32; 5];
    let (mut seq, mut sorted) = (Vec::new(), Vec::new());
    for len in 0..=5u32 {
        for code in 0..refs.len().pow(len) {
            seq.clear();
            let mut c = code;
            for _ in 0..len {
                seq.push(refs[c % refs.len()]);
                c /= refs.len();
            }
            let least = seq.iter().copied().filter(|n| fits(n)).min();
            let got = name_of(seq.iter().copied());
            assert_eq!(got, least.and_then(|n| name_of([n])), "{seq:?}");
            assert_eq!(got, name_of(seq.iter().rev().copied()), "{seq:?} reversed");
            sorted.clone_from(&seq);
            sorted.sort_unstable();
            assert_eq!(got, name_of(sorted.iter().copied()), "{seq:?} sorted");
            let below = |n: &&str| least.is_none_or(|l| *n < l);
            seen[0] += least.is_none() as u32;
            seen[1] += (least == Some("driver.")) as u32;
            seen[2] += least.is_some_and(|l| l.len() == NAME_MAX) as u32;
            seen[3] += seq.iter().any(|n| !n.starts_with("driver.") && below(n)) as u32;
            seen[4] += seq.iter().any(|n| n.len() > NAME_MAX && below(n)) as u32;
        }
    }
    assert!(seen.iter().all(|&n| n >= 9_000), "{seen:?}");
}

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

//! Every family the kernel reserves has its own generation, from 1 up and
//! never given twice, and the seal answers it for a supervisor's live family
//! alone: a family sealed and the next begun under the same supervisor
//! answer differently, which binds a network grant to one family.

use crate::foreign::seal::kind::LOCAL;
use super::fixture::family;
use crate::foreign::seal::generation::next;
use crate::foreign::seal::query::{generation, holds};
use crate::foreign::seal::table::LIVE;

const SUP: u32 = 0x6E40;

fn reserve(sup: u32) -> u64 {
    let mut f = family(sup);
    f.generation = next();
    let g = f.generation;
    LIVE.lock().push(f);
    g
}

fn end(sup: u32) {
    LIVE.lock().retain(|f| f.sup != sup);
}

#[test]
fn generations_rise_and_are_never_given_twice() {
    let (a, b, c) = (next(), next(), next());
    assert!(a >= 1 && a < b && b < c);
}

#[test]
fn the_next_family_under_one_supervisor_answers_another_generation() {
    assert_eq!(generation(SUP), 0, "no family, no generation");
    let first = reserve(SUP);
    assert_eq!(generation(SUP), first);
    end(SUP);
    assert_eq!(generation(SUP), 0);
    let second = reserve(SUP);
    assert!(second > first);
    assert_eq!(generation(SUP), second);
    end(SUP);
}

#[test]
fn a_measurement_is_held_only_by_the_live_family_that_recorded_it() {
    let m = [0x42; 32];
    assert!(!holds(SUP + 1, &m, LOCAL));
    reserve(SUP + 1);
    LIVE.lock().iter_mut().find(|f| f.sup == SUP + 1).unwrap().measured(&m, LOCAL);
    assert!(holds(SUP + 1, &m, LOCAL));
    end(SUP + 1);
    reserve(SUP + 1);
    assert!(!holds(SUP + 1, &m, LOCAL), "the next family recorded nothing");
    end(SUP + 1);
}

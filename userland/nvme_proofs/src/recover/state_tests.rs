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

//! The named states: each reads apart on the console, and only the two that
//! leave the controller working serve again (request_tests runs the rest).

use super::Recovered;

const ALL: [Recovered; 4] =
    [Recovered::Aborted, Recovered::Reset, Recovered::Gone, Recovered::Failed];

#[test]
fn every_state_has_its_own_words() {
    for (i, s) in ALL.iter().enumerate() {
        let w = s.words();
        assert!(!w.is_empty() && w.len() <= 60, "{s:?}: {w:?}");
        assert!(ALL[i + 1..].iter().all(|o| o.words() != w), "{w:?} names two states");
    }
}

#[test]
fn only_a_working_controller_serves_again() {
    for (s, serves) in ALL.into_iter().zip([true, true, false, false]) {
        assert_eq!(s.serves(), serves, "{s:?}");
    }
}

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

//! Which moves wait at all, and the sleep a wait becomes never cuts it
//! short.

use super::ready::EDGES;
use crate::device_power::DState;
use crate::pci_dsm::*;

#[test]
fn only_the_ways_back_to_d0_wait() {
    use DState::*;
    let all = [D0, D1, D2, D3Hot, D3Cold];
    for (from, to) in all.into_iter().flat_map(|f| all.map(|t| (f, t))) {
        let want = match (from, to) {
            (D1, D0) => Some(Wake::FromD1),
            (D2, D0) => Some(Wake::FromD2),
            (D3Hot, D0) => Some(Wake::FromD3Hot),
            (D3Cold, D0) => Some(Wake::FromD3Cold),
            _ => None,
        };
        assert_eq!(wake_of(from, to), want, "{from:?} -> {to:?}");
    }
}

#[test]
fn the_sleep_never_cuts_the_wait_short() {
    for us in EDGES {
        let ms = sleep_ms_for(us);
        assert!(ms * 1000 >= u64::from(us) + 1000, "{us} us slept as {ms} ms");
        assert!(ms * 1000 < u64::from(us) + 2000, "{us} us slept as {ms} ms, too long");
    }
}

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

//! F-103: an EC whose queries never end is drained at most 32 a call and
//! called a flood once past the storm bound; one a calm second never is.

use crate::ec::{drain, MAX_QUERIES};
use crate::ec_model::{EcModel, PORTS};
use crate::events::{Storms, RUNS, WINDOW_MS};

#[test]
fn a_query_flood_is_drained_in_bounded_steps_and_called_a_flood() {
    let (mut ec, mut storms) = (EcModel::new(0), Storms::default());
    ec.tricks.endless = true;
    let mut total = 0;
    let flooded = loop {
        let d = drain(&PORTS, &mut ec, &mut storms).unwrap();
        assert!(d.queries.len() <= MAX_QUERIES, "{}", d.queries.len());
        total += d.queries.len();
        if d.flooded {
            break true;
        }
        assert!(total <= RUNS as usize, "the bound holds");
    };
    assert!(flooded && total == RUNS as usize);
    let (mut calm, mut storms) = (EcModel::new(0), Storms::default());
    calm.tricks.endless = true;
    for second in 0..20 {
        calm.tricks.now = second * WINDOW_MS;
        assert!(!drain(&PORTS, &mut calm, &mut storms).unwrap().flooded, "32 a second is calm");
    }
}

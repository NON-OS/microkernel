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

//! Where the span ends: after a recovery and a present the device answered,
//! the count of recoveries in a row is kept at SERVED_MS less a millisecond
//! by the clock and starts over at SERVED_MS. Any span used to pass, from
//! some milliseconds to half a minute.

use super::client::flush;
use super::part::{answering_after_reset, Answer};
use super::recover_limit_tests::{present, RECOVERY};
use super::stall::{stalled, whole_flush, PRIMARY};
use crate::clock::{now_ms, pass};
use crate::protocol::E_INVAL;
use crate::server::recover::policy::SERVED_MS;
use crate::server::recover::settle;

#[test]
fn the_count_starts_over_at_served_ms_and_not_a_millisecond_before() {
    let (mut d, region, regs) = stalled();
    let _part = answering_after_reset(&region, &regs, Answer::Spec, RECOVERY + 1);
    settle(&mut d);
    assert_eq!(present(&mut d, &whole_flush()), 0, "the device answered a present");
    // A present refused before the device asks the clock nothing, so the
    // settle after it reads the time since the recovery to the millisecond;
    // an answered one's wait may ask some questions first.
    let refused = flush(PRIMARY.resource_id, PRIMARY.width, 0, 1, 1);
    let short = d.recovered_ms + SERVED_MS - 1;
    pass(short.checked_sub(now_ms()).expect("the present took less than the span"));
    assert_eq!(present(&mut d, &refused), E_INVAL);
    assert_eq!(d.in_a_row, 1, "the count started over a millisecond short of SERVED_MS");
    pass(1);
    assert_eq!(present(&mut d, &refused), E_INVAL);
    assert_eq!(d.in_a_row, 0, "the count kept at SERVED_MS");
}

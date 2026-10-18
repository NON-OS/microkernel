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

//! AUX transactions the channel cannot complete.

use super::aux_ch_tests::with_sink;
use crate::dp::aux::{Aux, AuxError};
use crate::engine::aux_ch::AuxCh;
use crate::engine::aux_regs::Ddi;
use crate::model::Ticks;

#[test]
fn a_channel_that_never_finishes_or_a_broken_clock_times_out() {
    let mut m = with_sink(vec![]);
    m.aux_stuck = true;
    let clock = Ticks::new();
    let mut r = [0u8; 20];
    let mut a = AuxCh { mmio: &mut m, ddi: Ddi::B, clock: &clock };
    assert_eq!(a.transfer(&[0x90, 0, 0, 0], &mut r), Err(AuxError::Timeout));
    let broken = Ticks { broken: true, ..Ticks::new() };
    let mut m = with_sink(vec![]);
    m.aux_stuck = true;
    assert_eq!(
        AuxCh { mmio: &mut m, ddi: Ddi::B, clock: &broken }.transfer(&[0x90, 0, 0, 0], &mut r),
        Err(AuxError::Timeout)
    );
}

#[test]
fn an_empty_or_oversized_request_is_refused_unsent() {
    let mut m = with_sink(vec![]);
    let clock = Ticks::new();
    let mut r = [0u8; 20];
    let mut a = AuxCh { mmio: &mut m, ddi: Ddi::B, clock: &clock };
    assert_eq!(a.transfer(&[], &mut r), Err(AuxError::Short));
    assert_eq!(a.transfer(&[0; 21], &mut r), Err(AuxError::Short));
    assert!(m.writes.is_empty());
}

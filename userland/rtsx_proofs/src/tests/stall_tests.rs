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

//! A data phase that never moves: named, slept through rather than spun,
//! the engines stopped, and CMD12 still sent to the card that is there.

use nonos_libc::{events, now_ms, sleeps};

use super::card_rw_tests::{ready, sent};
use super::rig::{knobs, sdhc};
use crate::card::read_blocks;
use crate::error::RtsxError;
use crate::model::Knobs;

#[test]
fn a_data_phase_that_never_moves_is_named_slept_through_and_stopped() {
    let (drv, card) = ready(sdhc(1));
    knobs(Knobs { stall_data: true, ..Knobs::default() });
    let (t0, naps, seen) = (now_ms(), sleeps(), events().len());
    assert_eq!(read_blocks(&drv, &card, 0, 4), Err(RtsxError::DataTimeout));
    assert!((10_000..10_100).contains(&(now_ms() - t0)) && sleeps() - naps >= 9_000);
    let after = events().split_off(seen);
    assert!(after.starts_with(&["stop cmd".into(), "stop dma".into()]), "{after:?}");
    assert_eq!(sent(), [18, 12]);
}

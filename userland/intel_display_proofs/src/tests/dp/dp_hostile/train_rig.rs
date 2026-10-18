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

//! One training against a hostile sink, held to its bounds and to what
//! its outcome must leave.

use super::profile::Profile;
use super::sink::Hostile;
use crate::dp::caps::Caps;
use crate::dp::dpcd::rate_code;
use crate::dp::native::RETRIES;
use crate::dp::train::{train, TrainError};
use crate::dp::train_budget::TRAIN_US;
use crate::dp::train_cr::CR_TRIES;

/// Twelve attempts; per attempt two link writes, the CR and EQ passes (a
/// status read of six bytes in up to six transactions and a write each),
/// their first writes and the pattern off; each transaction its retries.
pub fn bound() -> u32 {
    12 * RETRIES * (5 + 7 * (CR_TRIES + 5))
}

/// Outcome (0 trained, 1 failed, 2 AUX, 3 transmitter); status reads.
pub struct Seen {
    pub outcome: usize,
    pub status_reads: u32,
}

pub fn run(seed: u64, p: Profile, b: [u8; 16], max: (u32, u32)) -> usize {
    run_with(seed, p, b, max).outcome
}
pub fn run_with(seed: u64, p: Profile, b: [u8; 16], max: (u32, u32)) -> Seen {
    let caps = Caps::decode(&b).unwrap();
    let mut h = Hostile::new(seed, p, b, bound());
    let got = train(&mut h, &caps, max);
    assert!(h.waited_us <= TRAIN_US + 40_000, "{} us: past the time bound", h.waited_us);
    let outcome = match got {
        Ok(t) => {
            assert!(matches!(t.lanes, 1 | 2 | 4) && t.lanes <= caps.max_lanes.min(max.1));
            assert!(t.mbps <= caps.max_rate_mbps.min(max.0) && rate_code(t.mbps).is_some());
            assert_eq!(h.up, Some((t.mbps, t.lanes)), "the link reported is the one up");
            0
        }
        Err(e) => {
            assert_eq!(h.up, None, "a failure leaves the port down: {e:?}");
            match e {
                TrainError::Failed => 1,
                TrainError::Aux(_) => 2,
                TrainError::Transmitter => 3,
            }
        }
    };
    Seen { outcome, status_reads: h.status_reads }
}

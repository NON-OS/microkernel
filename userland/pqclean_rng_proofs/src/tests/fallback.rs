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

//! The weak fallback says so on the serial line, once a boot: never while
//! the hardware entropy source answers, once at the first weak value, and
//! not again however many weak values follow.

use crate::crypto::util::rng::global::{fill_with_fallback_secure, weak_u64};
use crate::stand_in::{entropy_works, SERIAL};

const LINE: &[u8] = b"[RNG] fallback used: weak bytes (entropy unavailable)\n";

fn times_said() -> usize {
    SERIAL.lock().unwrap().windows(LINE.len()).filter(|w| *w == LINE).count()
}

#[test]
fn the_fallback_is_said_once_and_only_when_it_is_weak() {
    entropy_works(true);
    let mut buf = [0u8; 20];
    fill_with_fallback_secure(&mut buf);
    assert_eq!(times_said(), 0, "hardware entropy answered");
    assert_eq!(buf[..8], 0x0123_4567_89ab_cdefu64.to_le_bytes());
    entropy_works(false);
    fill_with_fallback_secure(&mut buf);
    assert_eq!(times_said(), 1);
    (0..10).for_each(|_| {
        weak_u64();
    });
    fill_with_fallback_secure(&mut [0u8; 64]);
    assert_eq!(times_said(), 1, "once a boot");
}

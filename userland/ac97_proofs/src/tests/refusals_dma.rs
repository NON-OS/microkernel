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

//! A DMA grant refused, placed above 4 GiB, or short: both windows and the
//! claim given back, and the grant too when one was made.

use nonos_libc::script_dma;

use super::refusals::{refused, with_tail, HEAD, PORTS_BACK};
use crate::model::Options;

#[test]
fn a_refused_dma_grant_gives_both_windows_back() {
    script_dma(Err(-12));
    let flags = 6;
    let tail = with_tail(&[("pio_grant", 1), ("dma_map", flags)], &PORTS_BACK);
    assert_eq!(refused(Options::default()), with_tail(&HEAD, &tail));
}

// The engine names the list and every period in 32 bits; a grant above
// 4 GiB would be truncated and the engine would read some other memory.
#[test]
fn a_dma_grant_above_4_gib_or_short_is_given_back() {
    for answer in [(0xFFFF_F000, 0), (0x0010_0000, 0x1000)] {
        script_dma(Ok(answer));
        let got = refused(Options::default());
        assert_eq!(
            got[got.len() - 4..],
            [("dma_unmap", 70), PORTS_BACK[0], PORTS_BACK[1], PORTS_BACK[2]]
        );
    }
}

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

use super::port_rule::{counts, DOWNSTREAM, UPSTREAM};

const ROOT: u8 = 4;

#[test]
fn a_port_counts_by_what_it_says_or_by_the_switch_it_sits_in() {
    assert!(counts(Some(DOWNSTREAM), Some(Some(UPSTREAM))), "an honest port");
    assert!(counts(Some(UPSTREAM), Some(Some(UPSTREAM))), "one saying upstream on a switch's bus");
    assert!(counts(Some(ROOT), Some(Some(UPSTREAM))), "one saying root port there");
    assert!(counts(None, Some(Some(UPSTREAM))), "one with no type read");
    assert!(counts(None, Some(None)), "no type, below a bridge with none either");
    assert!(counts(Some(DOWNSTREAM), None), "it says so, wherever it sits");
    assert!(!counts(Some(ROOT), None), "a root port, nothing above it");
    assert!(!counts(None, None), "no type and nothing above: a root bus's bridge");
    assert!(!counts(Some(UPSTREAM), Some(Some(ROOT))), "a switch's upstream port");
    assert!(!counts(Some(UPSTREAM), Some(Some(DOWNSTREAM))), "a nested switch's upstream port");
}

// A seeded xorshift, as F-98's sweeps draw.
fn drawn(seed: u64) -> impl FnMut(u64) -> u64 {
    let mut x = seed | 1;
    move |n| {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x % n
    }
}

// A switch's ports as the walk reads them: mostly honest, else one lie.
fn told(pick: &mut impl FnMut(u64) -> u64) -> Option<u8> {
    [Some(DOWNSTREAM), Some(DOWNSTREAM), Some(UPSTREAM), Some(ROOT), None][pick(5) as usize]
}

// Every downstream port is counted whatever it says about itself; a root
// port or a switch's upstream port only when the port above it lied.
#[test]
fn every_switch_port_counts_whatever_it_reports() {
    let (mut lied, mut kept_out) = (0u32, 0u32);
    for seed in 1..=20_000u64 {
        let mut pick = drawn(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let parent = told(&mut pick);
        let nested = pick(2) == 0;
        // The upstream port of a switch below `parent`, or of one below a root port.
        let above_up = if nested { parent } else { Some(ROOT) };
        let up = counts(Some(UPSTREAM), Some(above_up));
        let port = told(&mut pick);
        assert!(counts(port, Some(Some(UPSTREAM))), "seed {seed}: {port:?}");
        lied += (port != Some(DOWNSTREAM)) as u32;
        if parent == Some(DOWNSTREAM) || !nested {
            assert!(!up, "seed {seed}: an upstream port below an honest port");
            kept_out += 1;
        }
        assert!(!counts(Some(ROOT), None), "seed {seed}");
    }
    assert!(lied > 8_000 && kept_out > 8_000, "{lied} lied, {kept_out} kept out");
}

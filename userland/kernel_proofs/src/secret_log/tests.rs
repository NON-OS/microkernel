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

use super::rule::{unlogged, DEBUG, SECRETS};
use std::collections::BTreeMap;

const ROOT: &str = "../..";

fn bits() -> BTreeMap<String, u64> {
    let defs = std::fs::read_to_string(format!("{ROOT}/src/capabilities/types/defs.rs")).unwrap();
    let mut out = BTreeMap::new();
    for l in defs.lines().map(str::trim) {
        if let Some((name, rest)) = l.split_once(" = 1 << ") {
            out.insert(
                name.to_string(),
                1u64 << rest.trim_end_matches(',').parse::<u32>().unwrap(),
            );
        }
    }
    out
}

#[test]
fn the_restated_bits_are_the_kernels() {
    let k = bits();
    assert_eq!(DEBUG, k["Debug"]);
    assert_eq!(SECRETS, k["InputSource"] | k["Keyring"] | k["DeviceSecret"] | k["InputConsumer"]);
}

#[test]
fn debug_is_withheld_from_every_secret_holder_off_dev() {
    let (mut x, mut withheld) = (11u64, 0);
    for i in 0..20_000u64 {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let caps = if i < 40 { 1 << i } else { x & ((1 << 40) - 1) };
        let std = unlogged(caps, false);
        assert!(std & DEBUG == 0 || std & SECRETS == 0, "{caps:#x}");
        assert_eq!(std & !DEBUG, caps & !DEBUG, "{caps:#x}: only Debug is touched");
        assert_eq!(std & DEBUG == 0, caps & DEBUG == 0 || caps & SECRETS != 0);
        assert_eq!(unlogged(caps, true), caps, "{caps:#x}: the dev profile keeps it");
        // The process table's flag is set exactly when Debug was offered
        // beside a secret (`withholds_debug`).
        assert_eq!(std != caps, caps & DEBUG != 0 && caps & SECRETS != 0);
        withheld += usize::from(std != caps);
    }
    println!("{withheld} masks had Debug withheld");
    assert!(withheld >= 4_000, "{withheld}");
}

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

//! The lines that quote a bound quote the one waited, every error fits a
//! console line, and the host table holds the hosts listed.

use crate::hosts::{is_host, is_ufs_class, HOSTS, INTEL};
use crate::ufs::facts::bounds::*;
use crate::ufs::host::hce::{HCE_STUCK_OFF, HCE_STUCK_ON};

#[test]
fn hce_lines_quote_their_bounds() {
    assert!(HCE_STUCK_ON.contains(&format!(" {HCE_OFF_MS} ms")));
    assert!(HCE_STUCK_OFF.contains(&format!(" {HCE_ON_MS} ms")));
    assert_eq!(HCE_ON_MS, 100, "the engine's own enable bound");
    for l in [HCE_STUCK_ON, HCE_STUCK_OFF] {
        assert!("driver.ufs: ".len() + l.len() < 160);
    }
}

#[test]
fn the_host_table_holds_the_listed_hosts() {
    assert_eq!(HOSTS.len(), 12);
    assert!(is_host(0x1B36, 0x0013), "QEMU");
    assert!(is_host(0x144D, 0xC00C), "Samsung");
    let intel: Vec<u16> = HOSTS.iter().filter(|(v, _)| *v == INTEL).map(|(_, d)| *d).collect();
    assert_eq!(
        intel,
        [0x9DFA, 0x4B41, 0x4B43, 0x98FA, 0x51FF, 0x54FF, 0x7E47, 0xA847, 0x7747, 0xE447]
    );
    assert!(!is_host(INTEL, 0x9D2B), "an eMMC host is sdhci's");
    assert!(is_ufs_class(0x01, 0x09));
    assert!(!is_ufs_class(0x01, 0x08) && !is_ufs_class(0x08, 0x05));
}

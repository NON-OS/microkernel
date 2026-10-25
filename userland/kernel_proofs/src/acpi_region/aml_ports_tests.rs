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

//! F-102: the fixed legacy ports AML may reach and the ones it may not.
//! Each allowed range has its reason, lies outside every kernel-owned
//! range, is granted on a machine where nobody holds it, and is refused
//! once a capsule does. The ports added to the kernel's list (the IMCR,
//! system control port A, the console's CRT pair, the debug ports) are
//! refused by name.

use super::machine::Machine;
use super::rule::{io, Legacy, Refusal, AML_PORTS, LEGACY_PORTS};

fn bare() -> Machine {
    Machine { map: vec![], windows: vec![], ports: vec![], hardware: vec![], pci: vec![], reset: vec![] }
}

#[test]
fn every_allowed_port_is_firmwares_until_a_capsule_holds_it() {
    for (lo, hi, why) in AML_PORTS {
        assert!(!why.is_empty() && lo < hi, "{lo:#x}");
        assert!(
            LEGACY_PORTS.iter().all(|(l, h, _)| hi <= *l || *h <= lo),
            "{lo:#x} is the kernel's"
        );
        assert_eq!(io(lo, hi - lo, &bare()), Ok(()), "{lo:#x}");
        let held = Machine { ports: vec![(lo as u16, hi as u16)], ..bare() };
        assert_eq!(io(lo, hi - lo, &held), Err(Refusal::Claimed), "{lo:#x}");
    }
}

#[test]
fn the_kernels_own_ports_are_refused_by_name() {
    for (port, which) in [
        (0x22, Legacy::Pic),
        (0x23, Legacy::Pic),
        (0x92, Legacy::Reset),
        (0xE9, Legacy::Debug),
        (0x3D4, Legacy::Console),
        (0x3D5, Legacy::Console),
        (0x402, Legacy::Debug),
        (0xCF9, Legacy::PciConfig),
    ] {
        assert_eq!(io(port, 1, &bare()), Err(Refusal::Legacy(which)), "{port:#x}");
    }
    let reasons: Vec<_> =
        LEGACY_PORTS.iter().map(|(_, _, l)| Refusal::Legacy(*l).reason()).collect();
    assert!(reasons.iter().all(|r| !r.is_empty()));
}

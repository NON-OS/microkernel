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

//! ACPI namespace device declarations (ANDD) and the path comparison that
//! ties a DMAR name to the one the ACPI tables spell.

use super::acpi_path::same_path;
use super::dmar_andd::{parse_andd, ANDD_TYPE, MAX_NAME};

fn andd(number: u8, name: &[u8]) -> Vec<u8> {
    let mut b = ANDD_TYPE.to_le_bytes().to_vec();
    b.extend_from_slice(&((8 + name.len() + 1) as u16).to_le_bytes());
    b.extend_from_slice(&[0, 0, 0, number]);
    b.extend_from_slice(name);
    b.push(0);
    b
}

#[test]
fn an_andd_gives_its_number_and_name() {
    let a = parse_andd(&andd(3, b"\\_SB.PCI0.SDHA")).unwrap();
    assert_eq!((a.number, a.name()), (3, &b"\\_SB.PCI0.SDHA"[..]));
}

#[test]
fn a_malformed_andd_is_refused() {
    let mut no_nul = andd(1, b"\\_SB.SDHA");
    no_nul.pop();
    assert_eq!(parse_andd(&no_nul), None);
    assert_eq!(parse_andd(&andd(1, b"")), None);
    assert_eq!(parse_andd(&andd(1, &[b'A'; MAX_NAME + 1])), None);
    assert_eq!(parse_andd(&andd(1, b"\\_SB.SD A")), None);
    let mut other = andd(1, b"\\_SB.SDHA");
    other[0] = 1;
    assert_eq!(parse_andd(&other), None);
    assert_eq!(parse_andd(&andd(1, b"\\_SB.SDHA")[..8]), None);
    assert!(parse_andd(&andd(1, &[b'A'; MAX_NAME])).is_some());
}

#[test]
fn paths_match_with_or_without_padding() {
    assert!(same_path(b"\\_SB.PCI0.SDHA", b"\\_SB_.PCI0.SDHA"));
    assert!(same_path(b"_SB.PCI0.SDHA", b"\\_SB.PCI0.SDHA"));
    assert!(!same_path(b"\\_SB.PCI0.SDHA", b"\\_SB.PCI0.SDHB"));
    assert!(!same_path(b"\\_SB.PCI0", b"\\_SB.PCI0.SDHA"));
    assert!(!same_path(b"\\_SB.PCI00.SDHA", b"\\_SB.PCI00.SDHA"));
    assert!(!same_path(b"", b""));
    assert!(!same_path(b"\\_SB..SDHA", b"\\_SB..SDHA"));
}

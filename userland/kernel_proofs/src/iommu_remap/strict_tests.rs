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

//! K3: remapping turns on with CFI clear, whatever firmware left set, and a
//! compatibility format message is never made while it is on.

use super::regs::{gcmd_strict, is_strict, GCMD_CFI, GCMD_IRE, GSTS_CFIS, GSTS_IRES};
use super::remap_rule::{outcome, Attempt, Outcome};

/// GSTS's other persistent bits: TES 31, QIES 26.
const TE_QIE: u32 = 1 << 31 | 1 << 26;

#[test]
fn every_command_clears_cfi_and_keeps_the_rest() {
    for persistent in [0, TE_QIE, TE_QIE | GCMD_CFI, GCMD_CFI] {
        let cmd = gcmd_strict(persistent, GCMD_IRE);
        assert_eq!(cmd & GCMD_CFI, 0, "CFI from {persistent:#x}");
        assert_eq!(cmd & TE_QIE, persistent & TE_QIE);
        assert_ne!(cmd & GCMD_IRE, 0);
    }
    assert_eq!(gcmd_strict(TE_QIE | GCMD_CFI, 0), TE_QIE, "CFI cleared alone");
}

#[test]
fn only_ire_without_cfi_is_strict() {
    assert!(is_strict(GSTS_IRES));
    assert!(is_strict(GSTS_IRES | TE_QIE));
    assert!(!is_strict(GSTS_IRES | GSTS_CFIS), "CFI passes compatibility format");
    assert!(!is_strict(0));
    assert!(!is_strict(GSTS_CFIS));
}

#[test]
fn a_compatibility_format_message_is_refused_under_remapping() {
    for opted_in in [false, true] {
        assert_eq!(outcome(Attempt::Routed, opted_in), Outcome::Remapped);
        assert_eq!(outcome(Attempt::Full, opted_in), Outcome::Full);
        assert_eq!(outcome(Attempt::Failed, opted_in), Outcome::Refused, "no fallback");
    }
    assert_eq!(outcome(Attempt::Off, false), Outcome::Refused);
    assert_eq!(outcome(Attempt::Off, true), Outcome::Compat, "the opt-in alone");
}

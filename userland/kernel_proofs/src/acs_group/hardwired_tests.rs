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

//! K11b (1): a bit the ACS capability does not offer counts as hard-wired,
//! as Linux drivers/pci/pci.c `pci_acs_flags_enabled` has it; a function
//! with no ACS capability redirects nothing.

use super::acs_rule::{enabling, redirects, CR, FUNCTION, PORT, RR, SV, UF};

#[test]
fn a_bit_the_capability_lacks_counts_as_hard_wired() {
    // A root port that does no peer-to-peer may offer Source Validation only.
    assert!(redirects(Some((SV, SV)), PORT));
    assert!(redirects(Some((SV | RR | CR, SV | RR | CR)), PORT), "no UF offered");
    assert!(redirects(Some((0, 0)), FUNCTION), "a capability offering nothing");
    assert!(!redirects(Some((SV, 0)), PORT), "offered but off");
    assert!(!redirects(Some((PORT, PORT & !UF)), PORT), "UF offered but off");
    assert!(!redirects(None, PORT), "no capability at all");
}

#[test]
fn enabling_turns_on_what_is_offered_and_keeps_the_rest() {
    assert_eq!(enabling(SV | UF, 0, PORT), SV | UF);
    assert_eq!(enabling(PORT, RR, PORT), PORT);
    assert_eq!(enabling(0, 1 << 6, PORT), 1 << 6, "other control bits are kept");
    assert_eq!(enabling(PORT, 0, FUNCTION), RR | CR, "only what is needed");
}

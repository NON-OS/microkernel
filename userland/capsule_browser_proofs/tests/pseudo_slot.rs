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

/* The cascade keeps one ::before/::after slot for every DOM node, and almost
 * all of them are empty. An empty slot must stay pointer-sized: at 1.3 KB it
 * held 14 MiB for an 11k-node page and ran the browser out of heap. */

use capsule_browser_proofs::browser::css::PseudoText;

#[test]
fn an_empty_pseudo_slot_costs_no_more_than_64_bytes() {
    let slot = core::mem::size_of::<(Option<PseudoText>, Option<PseudoText>)>();
    assert!(slot <= 64, "empty pseudo slot is {slot} bytes");
}
